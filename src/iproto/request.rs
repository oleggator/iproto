use crate::iproto::consts;
use rmp::encode;
use rmp::encode::ValueWriteError;
use rmp_serde::encode::Error;
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::io::Write;

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
        encode::write_pfix(wr, Self::REQUEST_TYPE).map_err(ValueWriteError::InvalidMarkerWrite)?;

        encode::write_pfix(wr, consts::IPROTO_SYNC).map_err(ValueWriteError::InvalidMarkerWrite)?;
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
        Call {
            request_id,
            procedure_name,
            args,
        }
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

        encode::write_pfix(wr, consts::IPROTO_EXPR).map_err(ValueWriteError::InvalidMarkerWrite)?;
        encode::write_str(wr, self.expression)?;

        encode::write_pfix(wr, consts::IPROTO_TUPLE)
            .map_err(ValueWriteError::InvalidMarkerWrite)?;
        rmp_serde::encode::write(wr, self.args)?;

        Ok(())
    }
}

pub struct Auth<'a> {
    request_id: usize,

    salt: &'a [u8],
    username: &'a str,
    password: Option<&'a str>,
}

impl<'a> Auth<'a> {
    pub fn new(
        request_id: usize,
        salt: &'a [u8],
        username: &'a str,
        password: Option<&'a str>,
    ) -> Self {
        Auth {
            request_id,
            salt,
            username,
            password,
        }
    }
}

impl<'a, W: Write> Request<W> for Auth<'a> {
    const REQUEST_TYPE: u8 = consts::IPROTO_AUTH;

    fn request_id(&self) -> usize {
        self.request_id
    }

    fn encode_body(&self, wr: &mut W) -> Result<(), Error> {
        let scramble = make_scramble(self.salt, self.password.unwrap_or(""));

        encode::write_map_len(wr, 2)?;

        encode::write_pfix(wr, consts::IPROTO_USER_NAME)
            .map_err(ValueWriteError::InvalidMarkerWrite)?;
        encode::write_str(wr, self.username)?;

        encode::write_pfix(wr, consts::IPROTO_TUPLE)
            .map_err(ValueWriteError::InvalidMarkerWrite)?;
        {
            encode::write_array_len(wr, 2)?;

            encode::write_str(wr, "chap-sha1")?;
            encode::write_str_len(wr, SCRAMBLE_SIZE as u32)?;
            wr.write_all(&scramble)
                .map_err(ValueWriteError::InvalidDataWrite)?;
        }

        Ok(())
    }
}

const SCRAMBLE_SIZE: usize = 20;

fn make_scramble(salt: &[u8], password: &str) -> [u8; SCRAMBLE_SIZE] {
    let mut sha1 = Sha1::new();

    sha1.update(password);
    let hash1 = sha1.finalize_reset();

    sha1.update(&hash1);
    let hash2 = sha1.finalize_reset();

    sha1.update(&salt[..SCRAMBLE_SIZE]);
    sha1.update(&hash2);
    let hash3 = sha1.finalize();

    let mut hash4 = [0; SCRAMBLE_SIZE];
    for (i, hash1_b) in hash1.iter().enumerate() {
        hash4[i] = hash1_b ^ hash3[i];
    }

    hash4
}
