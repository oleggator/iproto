use std::io::Read;
use rmp_serde::decode::Error;
use serde::de::DeserializeOwned;
use crate::iproto::consts;

#[derive(Debug)]
pub struct ResponseHeader {
    pub request_id: usize,
    pub response_code_indicator: u32,
}

impl ResponseHeader {
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, rmp_serde::decode::Error> {
        let mut request_id: Option<usize> = None;
        let mut response_code: Option<u32> = None;

        let map_len = rmp::decode::read_map_len(reader)?;
        for _ in 0..map_len {
            let code = rmp::decode::read_pfix(reader)?;
            match code {
                consts::RESPONSE_CODE_INDICATOR => {
                    response_code = Some(rmp::decode::read_int(reader)?);
                }
                consts::IPROTO_SYNC => {
                    request_id = Some(rmp::decode::read_u64(reader)? as usize);
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
            response_code_indicator: response_code.unwrap(),
        })
    }

    pub fn request_id(&self) -> usize {
        self.request_id
    }
    pub fn response_code_indicator(&self) -> u32 { self.response_code_indicator }
}

pub trait ResponseBody: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, rmp_serde::decode::Error>;
}

pub struct Response<B: ResponseBody> {
    header: ResponseHeader,
    body: B,
}

impl<B: ResponseBody> Response<B> {
    pub fn from_parts(header: ResponseHeader, body: B) -> Self {
        Self { header, body }
    }
}


pub struct CallResponse<D: DeserializeOwned> {
    data: D,
}

impl<D: DeserializeOwned> CallResponse<D> {
    pub fn into_data(self) -> D { self.data }
}

impl<D: DeserializeOwned> ResponseBody for CallResponse<D> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut data: Option<D> = None;

        let map_len = rmp::decode::read_map_len(reader)?;
        for _ in 0..map_len {
            let code = rmp::decode::read_pfix(reader)?;
            match code {
                consts::IPROTO_DATA => {
                    data = Some(rmp_serde::decode::from_read(reader.by_ref())?);
                }
                _ => {
                    let _: () = rmp_serde::decode::from_read(reader.by_ref())?;
                    panic!("invalid op");
                }
            }
        }

        Ok(Self {
            data: data.unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct ErrorExtra {
    error_type: String,
    error_file: String,
    error_line: u64,
    error_message: String,
    errno: u64,
    errcode: u64,
}


#[derive(Debug)]
pub struct ErrorResponse {
    error: String,
    error_extra: Option<ErrorExtra>,
}

impl ResponseBody for ErrorResponse {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        use rmp_serde::decode;

        let mut err: Option<String> = None;
        let mut error_extra: Option<ErrorExtra> = None;

        let map_len = rmp::decode::read_map_len(reader)?;
        for _ in 0..map_len {
            let code = rmp::decode::read_pfix(reader)?;
            match code {
                consts::IPROTO_ERROR_24 => {
                    err = Some(decode::from_read(reader.by_ref())?);
                }
                consts::IPROTO_ERROR => {
                    let error_extra_map_len = rmp::decode::read_map_len(reader)?;
                    assert_eq!(error_extra_map_len, 1);

                    let key = rmp::decode::read_pfix(reader)?;
                    assert_eq!(key, consts::MP_ERROR_STACK);

                    let error_stack_len = rmp::decode::read_array_len(reader)?;
                    assert_eq!(error_stack_len, 1);

                    let mut error_type: Option<String> = None;
                    let mut error_file: Option<String> = None;
                    let mut error_line: Option<u64> = None;
                    let mut error_message: Option<String> = None;
                    let mut errno: Option<u64> = None;
                    let mut errcode: Option<u64> = None;

                    let fields_n = rmp::decode::read_map_len(reader)?;
                    for _ in 0..fields_n {
                        let code = rmp::decode::read_pfix(reader)?;
                        match code {
                            consts::MP_ERROR_TYPE => {
                                error_type = Some(rmp_serde::decode::from_read(reader.by_ref())?);
                            }
                            consts::MP_ERROR_FILE => {
                                error_file = Some(rmp_serde::decode::from_read(reader.by_ref())?);
                            }
                            consts::MP_ERROR_LINE => {
                                error_line = Some(rmp::decode::read_int(reader)?);
                            }
                            consts::MP_ERROR_MESSAGE => {
                                error_message = Some(rmp_serde::decode::from_read(reader.by_ref())?);
                            }
                            consts::MP_ERROR_ERRNO => {
                                errno = Some(rmp::decode::read_int(reader)?);
                            }
                            consts::MP_ERROR_ERRCODE => {
                                errcode = Some(rmp::decode::read_int(reader)?);
                            }
                            _ => {
                                println!("invalid code: {}", code);
                            }
                        }
                    }

                    error_extra = Some(ErrorExtra {
                        error_type: error_type.unwrap(),
                        error_file: error_file.unwrap(),
                        error_line: error_line.unwrap(),
                        error_message: error_message.unwrap(),
                        errno: errno.unwrap(),
                        errcode: errcode.unwrap(),
                    });
                }
                _ => {
                    let _: () = decode::from_read(reader.by_ref())?;
                    panic!("invalid op");
                }
            }
        }

        Ok(Self {
            error: err.unwrap(),
            error_extra,
        })
    }
}

pub struct EmptyResponse;

impl ResponseBody for EmptyResponse {
    fn decode<R: Read>(_reader: &mut R) -> Result<Self, Error> {
        Ok(Self)
    }
}
