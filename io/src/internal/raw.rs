use std::io::{BufReader, Read};

use thiserror::Error;

use super::{lookup::LookupRef, value::Value};

pub trait CelesteIo: Sized {
    type Error;

    fn read<R: Read>(
        reader: &mut BufReader<R>,
        lookup: Option<LookupRef<'_>>,
    ) -> Result<Self, Self::Error>;
}

impl CelesteIo for bool {
    type Error = std::io::Error;

    fn read<R: Read>(
        reader: &mut BufReader<R>,
        _lookup: Option<LookupRef<'_>>,
    ) -> Result<Self, std::io::Error> {
        let mut buf = [0u8];
        reader.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }
}

macro_rules! impl_value_type_prim {
    ( $(( $size:literal, $x:ty )),* ) => {
        $(
            impl CelesteIo for $x {
                type Error = std::io::Error;

                fn read<R: Read>(
                    reader: &mut BufReader<R>,
                    _lookup: Option<LookupRef<'_>>,
                ) -> Result<Self, std::io::Error> {
                    let mut buf = [0u8; $size];
                    reader.read_exact(&mut buf)?;
                    Ok(<$x>::from_le_bytes(buf))
                }
            }
        )*
    };
}

impl_value_type_prim!((1, u8), (2, i16), (2, u16), (4, i32), (4, u32), (4, f32));

pub struct StringLength(pub usize);

impl CelesteIo for StringLength {
    type Error = std::io::Error;

    fn read<R: Read>(
        reader: &mut BufReader<R>,
        lookup: Option<LookupRef<'_>>,
    ) -> Result<Self, std::io::Error> {
        let mut result = 0usize;
        let mut bit_offset = 0usize;
        loop {
            let byte = u8::read(reader, lookup)?;
            result += ((byte & 0b0111_1111) as usize) << bit_offset;
            bit_offset += 7;
            if byte & 0b1000_0000 == 0 {
                return Ok(StringLength(result));
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum StringReadError {
    #[error("failed to read string bytes")]
    Io(#[from] std::io::Error),
    #[error("failed to decode string as utf")]
    Utf(#[from] std::string::FromUtf8Error),
}

pub struct NonRleString(pub String);

pub struct RleString(pub String);

impl From<NonRleString> for Value {
    fn from(x: NonRleString) -> Self {
        Value::String(x.0)
    }
}

impl From<RleString> for Value {
    fn from(x: RleString) -> Self {
        Value::String(x.0)
    }
}

impl CelesteIo for NonRleString {
    type Error = StringReadError;

    fn read<R: Read>(
        reader: &mut BufReader<R>,
        lookup: Option<LookupRef<'_>>,
    ) -> Result<Self, Self::Error> {
        let size = StringLength::read(reader, lookup)?.0;
        let mut bytes = Vec::with_capacity(size);
        for _ in 0..size {
            bytes.push(u8::read(reader, lookup)?);
        }
        Ok(NonRleString(String::from_utf8(bytes)?))
    }
}

impl CelesteIo for RleString {
    type Error = StringReadError;

    fn read<R: Read>(
        reader: &mut BufReader<R>,
        lookup: Option<LookupRef<'_>>,
    ) -> Result<Self, Self::Error> {
        let size = u16::read(reader, lookup)? as usize;
        let mut bytes = Vec::new();
        for _ in 0..size / 2 {
            let times = u8::read(reader, lookup)?;
            let byte = u8::read(reader, lookup)?;

            for _ in 0..times {
                bytes.push(byte);
            }
        }
        Ok(RleString(String::from_utf8(bytes)?))
    }
}
