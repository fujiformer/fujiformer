use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    io::{BufReader, Read},
};

use thiserror::Error;

use super::{
    lookup::{LookupError, LookupRef, LookupValue},
    raw::{CelesteIo, NonRleString, RleString, StringReadError},
};

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(x) => write!(f, "{}", x),
            Value::Int(x) => write!(f, "{}", x),
            Value::Float(x) => write!(f, "{}", x),
            Value::String(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Error, Debug)]
#[error("value does not match target conversion type")]
pub struct ValueConversionError;

impl TryFrom<Value> for i32 {
    type Error = ValueConversionError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(x) => Ok(x),
            _ => Err(ValueConversionError),
        }
    }
}

impl TryFrom<&Value> for i32 {
    type Error = ValueConversionError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(x) => Ok(*x),
            _ => Err(ValueConversionError),
        }
    }
}

impl TryFrom<Value> for u32 {
    type Error = ValueConversionError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(x) => x.try_into().map_err(|_| ValueConversionError),
            _ => Err(ValueConversionError),
        }
    }
}

impl TryFrom<&Value> for u32 {
    type Error = ValueConversionError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(x) => (*x).try_into().map_err(|_| ValueConversionError),
            _ => Err(ValueConversionError),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = ValueConversionError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(x) => Ok(x),
            _ => Err(ValueConversionError),
        }
    }
}

macro_rules! impl_value_from {
    ( $(( $var:ident, $x:ty )),* ) => {
        $(
            impl From<$x> for Value {
                fn from(x: $x) -> Self {
                    Value::$var((x).into())
                }
            }
        )*
    };
}

impl_value_from!(
    (Bool, bool),
    (Int, u8),
    (Int, i16),
    (Int, i32),
    (Float, f32),
    (String, String)
);

#[derive(Error, Debug)]
pub enum ReadValueError {
    #[error("failed to read value bytes")]
    Io(#[from] std::io::Error),
    #[error("failed to read string")]
    String(#[from] StringReadError),
    #[error("unknown value type {0}")]
    UnknownValueType(u8),
    #[error("lookup error")]
    LookupError(#[from] LookupError),
}

impl CelesteIo for Value {
    type Error = ReadValueError;

    fn read<R: Read>(
        reader: &mut BufReader<R>,
        lookup: Option<LookupRef<'_>>,
    ) -> Result<Self, Self::Error> {
        match u8::read(reader, lookup)? {
            0 => Ok(bool::read(reader, lookup)?.into()),
            1 => Ok(u8::read(reader, lookup)?.into()),
            2 => Ok(i16::read(reader, lookup)?.into()),
            3 => Ok(i32::read(reader, lookup)?.into()),
            4 => Ok(f32::read(reader, lookup)?.into()),
            5 => Ok(LookupValue::read(reader, lookup)?.into()),
            6 => Ok(NonRleString::read(reader, lookup)?.into()),
            7 => Ok(RleString::read(reader, lookup)?.into()),
            x => Err(ReadValueError::UnknownValueType(x)),
        }
    }
}
