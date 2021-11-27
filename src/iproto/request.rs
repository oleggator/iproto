use rmp::encode;
use std::io::Write;
use rmp::encode::ValueWriteError;
use serde::Serialize;
use crate::iproto::consts;


pub trait Request<W: Write> {
    const REQUEST_TYPE: u8;

    fn request_id(&self) -> usize;

    fn encode(&self, wr: &mut W) -> Result<(), rmp_serde::encode::Error> {
        self.encode_header(wr)?;
        self.encode_body(wr)?;
        Ok(())
    }

    fn encode_header(&self, wr: &mut W) -> Result<(), rmp_serde::encode::Error> {
        encode::write_map_len(wr, 2)?;

        encode::write_pfix(wr, consts::IPROTO_REQUEST_TYPE)
            .map_err(ValueWriteError::InvalidMarkerWrite)?;
        encode::write_pfix(wr, Self::REQUEST_TYPE)
            .map_err(ValueWriteError::InvalidMarkerWrite)?;

        encode::write_pfix(wr, consts::IPROTO_SYNC)
            .map_err(ValueWriteError::InvalidMarkerWrite)?;
        encode::write_u64(wr, self.request_id() as u64)?;

        Ok(())
    }

    fn encode_body(&self, wr: &mut W) -> Result<(), rmp_serde::encode::Error>;
}


pub struct Ping {
    request_id: usize,
}

impl<W: Write> Request<W> for Ping {
    const REQUEST_TYPE: u8 = consts::IPROTO_PING;

    fn request_id(&self) -> usize {
        self.request_id
    }

    fn encode_body(&self, wr: &mut W) -> Result<(), rmp_serde::encode::Error> {
        encode::write_map_len(wr, 0)?;
        Ok(())
    }
}


pub struct Call<'a, T: Serialize> {
    request_id: usize,
    procedure_name: &'a str,
    args: &'a T,
}

impl<'a, T: Serialize> Call<'a, T> {
    pub fn new(request_id: usize, procedure_name: &'a str, args: &'a T) -> Self {
        Call { request_id, procedure_name, args }
    }
}

impl<'a, T: Serialize, W: Write> Request<W> for Call<'a, T> {
    const REQUEST_TYPE: u8 = consts::IPROTO_CALL;

    fn request_id(&self) -> usize {
        self.request_id
    }

    fn encode_body(&self, wr: &mut W) -> Result<(), rmp_serde::encode::Error> {
        encode::write_map_len(wr, 2)?;

        encode::write_pfix(wr, consts::IPROTO_FUNCTION_NAME)
            .map_err(ValueWriteError::InvalidMarkerWrite)?;
        encode::write_str(wr, self.procedure_name)?;

        encode::write_pfix(wr, consts::IPROTO_TUPLE)
            .map_err(ValueWriteError::InvalidMarkerWrite)?;
        rmp_serde::encode::write(wr, self.args)?;

        Ok(())
    }
}


pub struct Eval<'a, T: Serialize> {
    request_id: usize,
    expression: &'a str,
    args: &'a T,
}

impl<'a, T: Serialize, W: Write> Request<W> for Eval<'a, T> {
    const REQUEST_TYPE: u8 = consts::IPROTO_CALL;

    fn request_id(&self) -> usize {
        self.request_id
    }

    fn encode_body(&self, wr: &mut W) -> Result<(), rmp_serde::encode::Error> {
        encode::write_map_len(wr, 2)?;

        encode::write_pfix(wr, consts::IPROTO_EXPR)
            .map_err(ValueWriteError::InvalidMarkerWrite)?;
        encode::write_str(wr, self.expression)?;

        encode::write_pfix(wr, consts::IPROTO_TUPLE)
            .map_err(ValueWriteError::InvalidMarkerWrite)?;
        rmp_serde::encode::write(wr, self.args)?;

        Ok(())
    }
}
