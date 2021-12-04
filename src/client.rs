use std::io::Cursor;
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, atomic::{AtomicU8, Ordering}};
use sharded_slab::{Pool, Slab};
use serde::Serialize;
use tokio::sync::{mpsc, oneshot, Notify};
use tokio::net::{TcpStream, ToSocketAddrs, tcp::{OwnedWriteHalf, OwnedReadHalf}};
use rmp_serde::decode;
use serde::de::DeserializeOwned;
use tokio::io::{BufReader, BufWriter};
use futures::future::try_join;
use crate::iproto::{consts, request, response};
use nix::sys::socket;

type Buffer = Vec<u8>;

#[derive(Debug)]
struct CursorRef {
    pub buffer_key: usize,
    pub position: u64,
}

struct RequestHandle {
    request_id: usize,
    tx: oneshot::Sender<CursorRef>,
}

pub struct Connection {
    state: AtomicU8,
    requests_to_process_tx: mpsc::Sender<usize>,

    pending_requests: Slab<RequestHandle>,
    requests_not_full_notify: Notify,

    buffer_pool: Arc<Pool<Buffer>>,

    mss: u32,
}

const DISCONNECTED_STATE: u8 = 0;
const CONNECTED_STATE: u8 = 0;

impl Connection {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<Arc<Self>> {
        use tokio::io::AsyncReadExt;

        let stream = TcpStream::connect(addr).await?;
        let mss = socket::getsockopt(stream.as_raw_fd(), socket::sockopt::TcpMaxSeg)?;

        let (mut read_stream, write_stream) = stream.into_split();
        {
            let mut greeting_raw = [0; 128];
            read_stream.read_exact(&mut greeting_raw).await?;
        }

        let (requests_to_process_tx, requests_to_process_rx) = mpsc::channel(128);
        let conn = Arc::new(Connection {
            state: AtomicU8::new(CONNECTED_STATE),
            requests_to_process_tx,
            pending_requests: Slab::new(),
            requests_not_full_notify: Notify::new(),
            buffer_pool: Arc::new(Pool::new()),
            mss,
        });

        let conn_clone = conn.clone();
        tokio::spawn(async move {
            let writer_task = conn_clone.writer(requests_to_process_rx, write_stream);
            let reader_task = conn_clone.reader(read_stream);
            match try_join(writer_task, reader_task).await {
                Ok(_) => {}
                Err(_) => {}
            }
        });

        Ok(conn)
    }

    fn write_req_to_buf<R>(&self, req: &R) -> Result<usize, rmp_serde::encode::Error>
        where R: request::Request<Buffer>,
    {
        let mut write_buf = self.buffer_pool.create().unwrap();

        // placeholder for body size (u32)
        write_buf.extend_from_slice(&[0xCE, 0, 0, 0, 0]);

        req.encode(write_buf.as_mut())?;

        let body_len = write_buf.len() - 5;
        write_buf[1] = (body_len >> 24) as u8;
        write_buf[2] = (body_len >> 16) as u8;
        write_buf[3] = (body_len >> 8) as u8;
        write_buf[4] = body_len as u8;

        Ok(write_buf.key())
    }

    pub async fn call<'a, T, R>(&self, name: &str, data: &T) -> std::io::Result<R>
        where
            T: Serialize,
            R: DeserializeOwned,
    {
        let (tx, rx) = oneshot::channel();
        let request_id = {
            let entry = self.pending_requests.vacant_entry().unwrap();
            let request_id = entry.key();
            entry.insert(RequestHandle { request_id, tx });
            request_id
        };

        let req = request::Call::new(request_id, name, data);
        let buffer_key = self.write_req_to_buf(&req).unwrap();
        self.requests_to_process_tx.send(buffer_key).await.unwrap();

        let CursorRef { buffer_key, position } = rx.await.unwrap();
        let buffer = self.buffer_pool.get(buffer_key).unwrap();
        let mut cursor: Cursor<&Buffer> = Cursor::new(buffer.as_ref());
        cursor.set_position(position);

        // decode body
        let map_len = rmp::decode::read_map_len(&mut cursor).unwrap();
        assert_eq!(map_len, 1);
        let code = rmp::decode::read_pfix(&mut cursor).unwrap();
        match code {
            consts::IPROTO_DATA => {
                let result = decode::from_read(cursor).unwrap();
                self.buffer_pool.clear(buffer_key);
                Ok(result)
            }
            consts::IPROTO_ERROR => {
                panic!("error");
            }
            _ => {
                panic!("invalid op");
            }
        }
    }

    async fn writer(&self, mut requests_to_process_rx: mpsc::Receiver<usize>, write_stream: OwnedWriteHalf) -> std::io::Result<()> {
        use tokio::io::AsyncWriteExt;

        let mut write_stream = BufWriter::with_capacity(128 * 1024, write_stream);

        while self.state.load(Ordering::Relaxed) == CONNECTED_STATE {
            let buffer_key = requests_to_process_rx.recv().await.unwrap();
            {
                let mut write_buf = self.buffer_pool.clone().get_owned(buffer_key).unwrap();
                write_stream.write_all(&mut write_buf).await?;
                self.buffer_pool.clear(buffer_key);
            }

            // TODO: change batching behaviour
            const OPTIMAL_PAYLOAD_SIZE: usize = 1000;
            while write_stream.buffer().len() < OPTIMAL_PAYLOAD_SIZE {
                if let Ok(buffer_key) = requests_to_process_rx.try_recv() {
                    let mut write_buf = self.buffer_pool.clone().get_owned(buffer_key).unwrap();
                    write_stream.write_all(&mut write_buf).await?;
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

        let mut read_stream = BufReader::with_capacity(128 * 1024, read_stream);

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
            let req = self.pending_requests.take(header.request_id()).unwrap();

            let cursor_ref = CursorRef {
                buffer_key,
                position: resp_reader.position(),
            };

            // resp_buf must be dropped before it was sent to prevent mutual access by receiver
            // (if receivers gets the key before it was dropped it receives null)
            drop(resp_buf);

            req.tx.send(cursor_ref).unwrap();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::timeout;
    use super::Connection;

    async fn conn() -> Arc<Connection> {
        Connection::connect("localhost:3301").await.unwrap()
    }

    #[tokio::test]
    async fn client_test() {
        let conn = conn().await;

        let (result, ): (usize, ) = timeout(Duration::from_secs(2), conn.call("sum", &(1, 2))).await.unwrap().unwrap();
        assert_eq!(result, 3);

        let (result, ): (usize, ) = timeout(Duration::from_secs(2), conn.call("sum", &(1, 2))).await.unwrap().unwrap();
        assert_eq!(result, 3);
    }

    #[tokio::test]
    async fn test_tarantool_error() {
        let conn = conn().await;

        let _: () = conn.call("not_existing_procedure", &(1, 2, 3)).await.unwrap();
    }
}
