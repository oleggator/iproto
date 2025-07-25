use std::io::Cursor;
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

use base64::Engine;
use base64::engine::general_purpose::STANDARD as Base64Engine;
use futures::FutureExt;
use futures::future::try_join;
use nix::sys::socket;
use serde::Serialize;
use serde::de::DeserializeOwned;
use sharded_slab::{Pool, Slab};
use thiserror::Error;
use tokio::io::{BufReader, BufWriter};
use tokio::net::{
    TcpStream, ToSocketAddrs,
    tcp::{OwnedReadHalf, OwnedWriteHalf},
};
use tokio::sync::{Notify, mpsc, oneshot, watch};

use crate::iproto::{consts, request, response};
use crate::utils::SlabEntryGuard;
use request::Request;
use response::ResponseBody;

const READ_BUFFER: usize = 128 * 1024;
const WRITE_BUFFER: usize = 128 * 1024;

// depends on the thread number
const REQ_CHANNEL_BUFFER: usize = 16 * 1024;

type Buffer = Vec<u8>;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("tarantool error")]
    TarantoolError(response::ErrorResponse),
    #[error("invalid response")]
    InvalidResponse,
    #[error("decoding error")]
    InvalidDecoding,
    #[error("error")]
    ErrorCode(u8),
    #[error("connection error")]
    ConnectionError(Arc<std::io::Error>),
}

#[derive(Debug)]
struct TarantoolResp {
    header: response::ResponseHeader,
    cursor_ref: CursorRef,
}

#[derive(Debug)]
struct CursorRef {
    buffer_key: usize,
    position: u64,
}

struct RequestHandle {
    request_id: usize,
    tx: oneshot::Sender<TarantoolResp>,
}

pub struct Connection {
    state: AtomicU8,
    requests_to_process_tx: mpsc::Sender<usize>,

    pending_requests: Slab<RequestHandle>,
    requests_not_full_notify: Notify,

    buffer_pool: Arc<Pool<Buffer>>,

    salt: Vec<u8>,
    mss: u32,

    error_rx: watch::Receiver<Option<Error>>,
}

const DISCONNECTED_STATE: u8 = 0;
const CONNECTED_STATE: u8 = 1;

impl Connection {
    pub async fn connect(addr: impl ToSocketAddrs) -> std::io::Result<Arc<Self>> {
        use tokio::io::AsyncReadExt;

        let stream = TcpStream::connect(addr).await?;
        let mss = socket::getsockopt(&stream, socket::sockopt::TcpMaxSeg)?;

        let (mut read_stream, write_stream) = stream.into_split();
        let salt = {
            let mut greeting_raw = [0; 128];
            read_stream.read_exact(&mut greeting_raw).await?;
            let salt_b64 = std::str::from_utf8(&greeting_raw[64..108]).unwrap().trim();

            Base64Engine.decode(salt_b64).unwrap()
        };

        let (requests_to_process_tx, requests_to_process_rx) = mpsc::channel(REQ_CHANNEL_BUFFER);
        let (error_tx, error_rx) = tokio::sync::watch::channel(None);

        let conn = Arc::new(Connection {
            state: AtomicU8::new(CONNECTED_STATE),
            requests_to_process_tx,
            pending_requests: Slab::new(),
            requests_not_full_notify: Notify::new(),
            buffer_pool: Arc::new(Pool::new()),
            salt,
            mss,
            error_rx,
        });

        let writer_conn = conn.clone();
        let writer_task = tokio::task::Builder::new()
            .name("writer")
            .spawn(async move {
                writer_conn
                    .writer(requests_to_process_rx, write_stream)
                    .await
            })?;

        let reader_conn = conn.clone();
        let reader_task = tokio::task::Builder::new()
            .name("reader")
            .spawn(async move { reader_conn.reader(read_stream).await })?;

        tokio::task::Builder::new()
            .name("iproto error catcher")
            .spawn(async move {
                match try_join(writer_task, reader_task).await.unwrap() {
                    (Ok(()), Ok(())) => {}
                    (Err(err), _) | (_, Err(err)) => {
                        let _ = error_tx.send(Some(Error::ConnectionError(Arc::new(err))));
                    }
                }
            })?;

        Ok(conn)
    }

    fn write_req_to_buf<R>(&self, req: &R) -> Result<usize, rmp_serde::encode::Error>
    where
        R: Request<Buffer>,
    {
        let mut write_buf = self.buffer_pool.create().unwrap();

        // placeholder for body size (u32)
        write_buf.extend_from_slice(&[0xCE, 0, 0, 0, 0]);

        req.encode(write_buf.as_mut())?;

        let body_len = write_buf.len() as u32 - 5;
        write_buf[1] = (body_len >> 24) as u8;
        write_buf[2] = (body_len >> 16) as u8;
        write_buf[3] = (body_len >> 8) as u8;
        write_buf[4] = body_len as u8;

        Ok(write_buf.key())
    }

    async fn make_request_inner<Req, Resp, F>(&self, f: F) -> Result<Resp, Error>
    where
        Req: Request<Buffer>,
        Resp: response::ResponseBody,
        F: FnOnce(usize) -> Req,
    {
        let TarantoolResp {
            header:
                response::ResponseHeader {
                    response_code_indicator,
                    ..
                },
            cursor_ref:
                CursorRef {
                    buffer_key,
                    position,
                },
        } = {
            let (tx, rx) = oneshot::channel();
            let request_id = {
                let entry = self.pending_requests.vacant_entry().unwrap();
                let request_id = entry.key();
                entry.insert(RequestHandle { request_id, tx });
                request_id
            };

            let _guard = SlabEntryGuard::new(request_id, &self.pending_requests);

            let req = f(request_id);
            let buffer_key = self.write_req_to_buf(&req).unwrap();
            self.requests_to_process_tx.send(buffer_key).await.unwrap();

            rx.await.unwrap()
        };

        let buffer = self.buffer_pool.get(buffer_key).unwrap();
        let mut cursor: Cursor<&Buffer> = Cursor::new(buffer.as_ref());
        cursor.set_position(position);

        const IPROTO_OK: u32 = consts::IPROTO_OK as u32;
        let result = match response_code_indicator {
            IPROTO_OK => {
                let data_resp = Resp::decode(&mut cursor).unwrap();
                Ok(data_resp)
            }
            0x8000..=0x8fff => {
                let _error_code = response_code_indicator - 0x8000;
                let err_resp = response::ErrorResponse::decode(&mut cursor).unwrap();
                Err(Error::TarantoolError(err_resp))
            }
            _ => {
                panic!("error")
            }
        };
        self.buffer_pool.clear(buffer_key);

        result
    }

    fn await_err(&self) -> impl Future<Output = Error> {
        let mut error_rx = self.error_rx.clone();

        async move {
            if let Some(err) = error_rx.borrow().as_ref() {
                return err.clone();
            }

            let result = error_rx.changed().await;
            result.unwrap();
            error_rx.borrow().clone().unwrap()
        }
    }

    pub async fn make_request<Req, Resp, F>(&self, f: F) -> Result<Resp, Error>
    where
        Req: Request<Buffer>,
        Resp: response::ResponseBody,
        F: FnOnce(usize) -> Req,
    {
        use futures_lite::FutureExt;
        self.make_request_inner(f)
            .or(self.await_err().map(Err))
            .await
    }

    pub async fn call<T, R>(&self, name: &str, data: &T) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let resp: response::CallResponse<R> = self
            .make_request(|request_id| request::Call::new(request_id, name, data))
            .await?;
        Ok(resp.into_data())
    }

    pub async fn auth(&self, username: &str, password: Option<&str>) -> Result<(), Error> {
        let _resp: response::EmptyResponse = self
            .make_request(|request_id| {
                request::Auth::new(request_id, &self.salt, username, password)
            })
            .await?;
        Ok(())
    }

    async fn writer(
        &self,
        mut requests_to_process_rx: mpsc::Receiver<usize>,
        write_stream: OwnedWriteHalf,
    ) -> std::io::Result<()> {
        use tokio::io::AsyncWriteExt;

        let mut write_stream = BufWriter::with_capacity(WRITE_BUFFER, write_stream);

        while self.state.load(Ordering::Relaxed) == CONNECTED_STATE {
            let buffer_key = requests_to_process_rx.recv().await.unwrap();
            {
                let write_buf = self.buffer_pool.clone().get_owned(buffer_key).unwrap();
                write_stream.write_all(&write_buf).await?;
                self.buffer_pool.clear(buffer_key);
            }

            // TODO: change batching behaviour
            const OPTIMAL_PAYLOAD_SIZE: usize = 1000;
            while write_stream.buffer().len() < OPTIMAL_PAYLOAD_SIZE {
                if let Ok(buffer_key) = requests_to_process_rx.try_recv() {
                    let write_buf = self.buffer_pool.clone().get_owned(buffer_key).unwrap();
                    write_stream.write_all(&write_buf).await?;
                    self.buffer_pool.clear(buffer_key);
                } else {
                    break;
                }
            }

            write_stream.flush().await?;
        }

        Ok(())
    }

    async fn reader(&self, read_stream: OwnedReadHalf) -> std::io::Result<()> {
        use tokio::io::AsyncReadExt;

        let mut read_stream = BufReader::with_capacity(READ_BUFFER, read_stream);

        let mut payload_len_raw = [0; 5];
        while self.state.load(Ordering::Relaxed) == CONNECTED_STATE {
            read_stream.read_exact(&mut payload_len_raw).await?;

            if payload_len_raw[0] != 0xCE {
                panic!("invalid resp");
            }

            let len = ((payload_len_raw[1] as usize) << 24)
                + ((payload_len_raw[2] as usize) << 16)
                + ((payload_len_raw[3] as usize) << 8)
                + (payload_len_raw[4] as usize);

            let mut resp_buf = self.buffer_pool.clone().create_owned().unwrap();
            let buffer_key = resp_buf.key();
            resp_buf.resize(len, 0);
            read_stream.read_exact(&mut resp_buf).await?;

            let resp_buf_ref: &mut Buffer = resp_buf.as_mut();
            let mut resp_reader = Cursor::new(resp_buf_ref);

            let header = response::ResponseHeader::decode(&mut resp_reader).unwrap();
            let request_id = header.request_id();

            let result = TarantoolResp {
                header,
                cursor_ref: CursorRef {
                    buffer_key,
                    position: resp_reader.position(),
                },
            };

            // resp_buf must be dropped before it was sent to prevent mutual access by receiver
            // (if receivers gets the key before it was dropped it receives null)
            drop(resp_buf);

            let req = self.pending_requests.take(request_id).unwrap();
            req.tx.send(result).unwrap();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Connection;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::timeout;

    const TESTING_HOST: &str = "localhost:3301";

    async fn conn() -> Arc<Connection> {
        Connection::connect(TESTING_HOST).await.unwrap()
    }

    #[tokio::test]
    async fn client_test() {
        let conn = conn().await;
        let t = Duration::from_secs(2);

        timeout(t, conn.auth("guest", None)).await.unwrap().unwrap();

        let (result,): (usize,) = timeout(t, conn.call("sum", &(1, 2)))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result, 3);

        let (result,): (usize,) = timeout(t, conn.call("sum", &(1, 2)))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result, 3);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_tarantool_error() {
        let conn = conn().await;
        let _: () = conn
            .call("not_existing_procedure", &(1, 2, 3))
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_user() {
        let conn = conn().await;
        conn.auth("kek", None).await.unwrap();
    }
}
