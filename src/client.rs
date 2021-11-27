use std::io::Cursor;
use std::sync::{Arc, atomic::{AtomicU8, Ordering}};
use sharded_slab::Slab;
use serde::Serialize;
use tokio::sync::{mpsc, oneshot, Notify, Mutex};
use tokio::net::{TcpStream, ToSocketAddrs, tcp::{OwnedWriteHalf, OwnedReadHalf}};
use rmp_serde::decode;
use serde::de::{DeserializeOwned, IgnoredAny};
use tokio::io::{BufReader, BufWriter};
use futures::future::try_join;
use crate::iproto::{consts, request};


struct RequestHandle {
    request_id: usize,
    tx: oneshot::Sender<Cursor<Vec<u8>>>,
}

pub struct Connection {
    state: AtomicU8,
    requests_to_process_tx: mpsc::Sender<usize>,

    requests: Slab<RequestHandle>,
    requests_not_full_notify: Notify,

    write_buffer: Mutex<Vec<u8>>,
}

const DISCONNECTED_STATE: u8 = 0;
const CONNECTED_STATE: u8 = 0;

impl Connection {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<Arc<Self>> {
        use tokio::io::AsyncReadExt;

        let stream = TcpStream::connect(addr).await?;
        let (mut read_stream, write_stream) = stream.into_split();
        {
            let mut greeting_raw = [0; 128];
            read_stream.read_exact(&mut greeting_raw).await?;
        }

        let (requests_to_process_tx, requests_to_process_rx) = mpsc::channel(128);
        let conn = Arc::new(Connection {
            state: AtomicU8::new(CONNECTED_STATE),
            requests_to_process_tx,
            requests: Slab::new(),
            requests_not_full_notify: Notify::new(),
            write_buffer: Mutex::new(Vec::new()),
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

    async fn write_req_to_buf<R>(&self, req: &R) -> Result<(), rmp_serde::encode::Error>
        where R: request::Request<Vec<u8>>,
    {
        let mut write_buf = self.write_buffer.lock().await;
        let write_buf: &mut Vec<u8> = write_buf.as_mut();
        let begin = write_buf.len();

        // placeholder for body size (u32)
        write_buf.extend_from_slice(&[0xCE, 0, 0, 0, 0]);

        req.encode(write_buf)?;

        let body_len = write_buf.len() - 5 - begin;
        write_buf[begin + 1] = (body_len >> 24) as u8;
        write_buf[begin + 2] = (body_len >> 16) as u8;
        write_buf[begin + 3] = (body_len >> 8) as u8;
        write_buf[begin + 4] = body_len as u8;

        Ok(())
    }

    pub async fn call<'a, T, R>(&self, name: &str, data: &T) -> std::io::Result<R>
        where
            T: Serialize,
            R: DeserializeOwned,
    {
        let (tx, rx) = oneshot::channel();
        let request_id = {
            let entry = self.requests.vacant_entry().unwrap();
            let request_id = entry.key();
            entry.insert(RequestHandle { request_id, tx });
            request_id
        };

        let req = request::Call::new(request_id, name, data);
        self.write_req_to_buf(&req).await.unwrap();
        self.requests_to_process_tx.send(request_id).await.unwrap();

        let mut cursor = rx.await.unwrap();

        // decode body
        let map_len = rmp::decode::read_map_len(&mut cursor).unwrap();
        assert_eq!(map_len, 1);
        let code = rmp::decode::read_pfix(&mut cursor).unwrap();
        match code {
            consts::IPROTO_DATA => {
                let result = decode::from_read(cursor).unwrap();
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
            let _request_id = match requests_to_process_rx.recv().await {
                Some(request_id) => request_id,
                None => break,
            };

            let mut write_buf = self.write_buffer.lock().await;

            write_stream.write_all(write_buf.as_mut()).await?;
            write_stream.flush().await?;

            write_buf.truncate(0);
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

            let mut resp_buf: Vec<u8> = vec![0; len];
            read_stream.read_exact(&mut resp_buf).await?;

            let mut request_id: Option<usize> = None;
            let mut _request_code: Option<u32> = None;

            let mut resp_reader = Cursor::new(resp_buf);

            // decode header
            let map_len = rmp::decode::read_map_len(&mut resp_reader).unwrap();
            for _ in 0..map_len {
                let code = rmp::decode::read_pfix(&mut resp_reader).unwrap();
                match code {
                    consts::IPROTO_SYNC => {
                        request_id = Some(rmp::decode::read_u64(&mut resp_reader).unwrap() as usize);
                    }
                    consts::IPROTO_REQUEST_TYPE => {
                        _request_code = Some(rmp::decode::read_u32(&mut resp_reader).unwrap());
                    }
                    consts::IPROTO_SCHEMA_VERSION => {
                        rmp::decode::read_u32(&mut resp_reader).unwrap();
                    }
                    _ => {
                        let _: IgnoredAny = rmp_serde::decode::from_read(&mut resp_reader).unwrap();
                    }
                }
            }

            let request_id = request_id.unwrap();
            let req = self.requests.take(request_id).unwrap();
            req.tx.send(resp_reader).unwrap();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn client_test() {
        let conn = crate::client::Connection::connect("localhost:3301").await.unwrap();

        let (result, ): (usize, ) = timeout(Duration::from_secs(2), conn.call("sum", &(1, 2))).await.unwrap().unwrap();
        assert_eq!(result, 3);

        let (result, ): (usize, ) = timeout(Duration::from_secs(2), conn.call("sum", &(1, 2))).await.unwrap().unwrap();
        assert_eq!(result, 3);
    }
}
