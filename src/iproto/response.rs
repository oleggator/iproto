use std::io::Read;
use serde::de::DeserializeOwned;
use crate::iproto::consts;

pub struct ResponseHeader {
    request_id: usize,
}

impl ResponseHeader {
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, rmp_serde::decode::Error> {
        let mut request_id: Option<usize> = None;

        let map_len = rmp::decode::read_map_len(reader)?;
        for _ in 0..map_len {
            let code = rmp::decode::read_pfix(reader)?;
            match code {
                consts::IPROTO_SYNC => {
                    request_id = Some(rmp::decode::read_u64(reader)? as usize);
                }
                consts::IPROTO_REQUEST_TYPE => {
                    let _ = rmp::decode::read_u32(reader)?;
                }
                consts::IPROTO_SCHEMA_VERSION => {
                    rmp::decode::read_u32(reader)?;
                }
                _ => {
                    panic!("invalid code");
                }
            }
        }

        Ok(ResponseHeader {
            request_id: request_id.unwrap(),
        })
    }

    pub fn request_id(&self) -> usize {
        self.request_id
    }
}

pub trait Response<R: Read, B: DeserializeOwned>: Sized {
    const REQUEST_TYPE: u8;

    fn from_parts(header: ResponseHeader, body: B) -> Self;

    fn decode(reader: &mut R) -> Result<Self, rmp_serde::decode::Error> {
        let header = Self::decode_header(reader)?;
        let body = Self::decode_body(reader)?;

        Ok(Self::from_parts(header, body))
    }

    fn decode_header(reader: &mut R) -> Result<ResponseHeader, rmp_serde::decode::Error> {
        ResponseHeader::decode(reader)
    }

    fn decode_body(reader: &mut R) -> Result<B, rmp_serde::decode::Error> {
        rmp_serde::decode::from_read(reader)
    }
}
