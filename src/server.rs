use crate::iproto::consts;
use futures::future::try_join;
use sharded_slab::Pool;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::sync::mpsc::Sender;

pub async fn serve<A: ToSocketAddrs>(addr: A) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        let conn = Connection::new();
        tokio::spawn(async move {
            conn.process(socket, addr).await.unwrap();
        });
    }
}

type Buffer = Vec<u8>;

struct Connection {
    buffer_pool: Arc<Pool<Buffer>>,
}

impl Connection {
    pub fn new() -> Arc<Connection> {
        Arc::new(Connection {
            buffer_pool: Arc::new(Pool::new()),
        })
    }

    pub async fn process<A: ToSocketAddrs>(
        self: Arc<Self>,
        mut socket: TcpStream,
        _addr: A,
    ) -> std::io::Result<()> {
        {
            let greeting = "Tarantool 2.8.2 (Binary) e5b9ac86-81bd-4042-b5f8-2e37bcfe4f38";
            let salt = "bCgy7N2ASRxzpE3XXdIzpBOizz+RA7z+actQZSaUOf8=";
            let body = format!("{:<63}\n{:<63}\n", greeting, salt);
            socket.write_all(body.as_bytes()).await?;
            socket.flush().await?;
        }

        let (req_s, req_r) = async_channel::bounded(128);
        let (resp_s, resp_r) = tokio::sync::mpsc::channel(128);

        let mut workers = vec![];
        for _ in 0..64 {
            let resp_s = resp_s.clone();
            let req_r = req_r.clone();
            let conn = self.clone();
            workers.push(tokio::spawn(async move {
                conn.handler(req_r, resp_s).await.unwrap();
            }));
        }

        let (read_stream, write_stream) = socket.into_split();

        let writer_task = self.writer(write_stream, resp_r);
        let reader_task = self.clone().reader(read_stream, req_s);

        try_join(reader_task, writer_task).await.unwrap();

        Ok(())
    }

    async fn handler(
        self: Arc<Self>,
        req_r: async_channel::Receiver<usize>,
        resp_s: Sender<usize>,
    ) -> std::io::Result<()> {
        loop {
            let request_id = {
                let mut request_type: Option<u8> = None;
                let mut request_id: Option<u64> = None;

                let buffer_key = req_r.recv().await.unwrap();
                let mut buf = self.buffer_pool.clone().get_owned(buffer_key).unwrap();

                use rmp::decode::*;
                let mut body_reader: &[u8] = &mut buf;

                // header
                let map_len = read_map_len(&mut body_reader).unwrap();
                for _ in 0..map_len {
                    let code = read_pfix(&mut body_reader).unwrap();
                    match code {
                        consts::IPROTO_REQUEST_TYPE => {
                            request_type = Some(read_pfix(&mut body_reader).unwrap());
                        }
                        consts::IPROTO_SYNC => {
                            request_id = Some(read_int(&mut body_reader).unwrap());
                        }
                        _ => {
                            return Ok(());
                        }
                    }
                }

                match request_type.unwrap() {
                    consts::IPROTO_CALL => {
                        let mut procedure_name: Option<String> = None;
                        let mut tuple: Option<rmpv::Value> = None;

                        let map_len = read_map_len(&mut body_reader).unwrap();
                        for _ in 0..map_len {
                            let code = read_pfix(&mut body_reader).unwrap();
                            match code {
                                consts::IPROTO_FUNCTION_NAME => {
                                    procedure_name = Some(
                                        rmp_serde::decode::from_read(&mut body_reader).unwrap(),
                                    );
                                }
                                consts::IPROTO_TUPLE => {
                                    tuple =
                                        Some(rmpv::decode::read_value(&mut body_reader).unwrap());
                                }
                                _ => {
                                    return Ok(());
                                }
                            }
                        }
                    }
                    consts::IPROTO_PING => {}
                    _ => {
                        return Ok(());
                    }
                }

                self.buffer_pool.clone().clear(buffer_key);
                request_id.unwrap()
            };

            let buffer_key = {
                use rmp::encode::*;
                let mut buf = self.buffer_pool.clone().create_owned().unwrap();

                let mut write_buffer_writer: &mut Vec<u8> = &mut buf;
                {
                    write_map_len(&mut write_buffer_writer, 2).unwrap();

                    write_pfix(&mut write_buffer_writer, consts::RESPONSE_CODE_INDICATOR).unwrap();
                    write_pfix(&mut write_buffer_writer, consts::IPROTO_OK).unwrap();

                    write_pfix(&mut write_buffer_writer, consts::IPROTO_SYNC).unwrap();
                    write_u64(&mut write_buffer_writer, request_id).unwrap();
                }
                {
                    write_map_len(&mut write_buffer_writer, 1).unwrap();

                    write_pfix(&mut write_buffer_writer, consts::IPROTO_DATA).unwrap();
                    rmp_serde::encode::write(&mut write_buffer_writer, &(3,)).unwrap();
                }

                buf.key()
            };
            resp_s.send(buffer_key).await.unwrap();
        }
    }

    async fn reader(
        self: Arc<Self>,
        read_stream: OwnedReadHalf,
        req_s: async_channel::Sender<usize>,
    ) -> std::io::Result<()> {
        let mut read_stream = BufReader::new(read_stream);

        let mut size_raw = [0; 5];
        loop {
            let buffer_key = {
                let mut buf = self.buffer_pool.clone().create_owned().unwrap();
                let body_size = {
                    read_stream.read_exact(&mut size_raw).await.unwrap();
                    let mut size_reader: &[u8] = &mut size_raw;
                    rmp::decode::read_int(&mut size_reader).unwrap()
                };
                buf.resize(body_size, 0);
                read_stream.read_exact(&mut buf).await.unwrap();

                buf.key()
            };

            req_s.send(buffer_key).await.unwrap();
        }
    }

    async fn writer(
        &self,
        write_stream: OwnedWriteHalf,
        mut responses_to_send: tokio::sync::mpsc::Receiver<usize>,
    ) -> std::io::Result<()> {
        let mut write_stream = BufWriter::new(write_stream);

        let mut body_len = [0; 5];

        loop {
            let write_buffer_key = responses_to_send.recv().await.unwrap();
            {
                let write_buffer = self
                    .buffer_pool
                    .clone()
                    .get_owned(write_buffer_key)
                    .unwrap();
                let mut body_len_writer: &mut [u8] = &mut body_len;
                rmp::encode::write_u32(&mut body_len_writer, write_buffer.len() as u32).unwrap();
                write_stream.write_all(&body_len).await.unwrap();
                write_stream.write_all(&write_buffer).await.unwrap();

                self.buffer_pool.clear(write_buffer_key);
            }

            const OPTIMAL_PAYLOAD_SIZE: usize = 1000;
            while write_stream.buffer().len() < OPTIMAL_PAYLOAD_SIZE {
                if let Ok(write_buffer_key) = responses_to_send.try_recv() {
                    let write_buffer = self
                        .buffer_pool
                        .clone()
                        .get_owned(write_buffer_key)
                        .unwrap();
                    let mut body_len_writer: &mut [u8] = &mut body_len;
                    rmp::encode::write_u32(&mut body_len_writer, write_buffer.len() as u32)
                        .unwrap();
                    write_stream.write_all(&body_len).await.unwrap();
                    write_stream.write_all(&write_buffer).await.unwrap();

                    self.buffer_pool.clear(write_buffer_key);
                } else {
                    break;
                }
            }

            write_stream.flush().await?;
        }
    }
}
