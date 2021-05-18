use std::io::{BufReader, Read};

use super::{raw::CelesteIo, value::Value};

use thiserror::Error;

pub struct Lookup(Vec<String>);

impl Lookup {
    pub fn new(x: Vec<String>) -> Self {
        Lookup(x)
    }

    pub fn as_ref(&self) -> LookupRef<'_> {
        LookupRef(&self.0)
    }
}

#[derive(Clone, Copy)]
pub struct LookupRef<'a>(&'a [String]);

impl<'a> LookupRef<'a> {
    pub fn get(&self, i: usize) -> Option<&str> {
        self.0.get(i).map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub struct LookupValue(pub String);

impl From<LookupValue> for Value {
    fn from(x: LookupValue) -> Self {
        Value::String(x.0)
    }
}

#[derive(Error, Debug)]
pub enum LookupError {
    #[error("failed to read bytes")]
    Io(#[from] std::io::Error),
    #[error("missing lookup")]
    MissingLookup,
    #[error("lookup out of bounds (expected 0..{length}, got {index})")]
    OutOfBounds { length: usize, index: usize },
}

impl CelesteIo for LookupValue {
    type Error = LookupError;

    fn read<R: Read>(
        reader: &mut BufReader<R>,
        lookup: Option<LookupRef<'_>>,
    ) -> Result<Self, Self::Error> {
        let index = u16::read(reader, None)? as usize;
        let lookup = lookup.ok_or(LookupError::MissingLookup)?;
        lookup
            .get(index)
            .map(str::to_string)
            .map(LookupValue)
            .ok_or_else(|| LookupError::OutOfBounds {
                length: lookup.len(),
                index,
            })
    }
}
