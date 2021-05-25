use std::io::{BufReader, Read};

use thiserror::Error;

use crate::{
    filler::{decode_fillers, Filler, FillersDecodeError},
    internal::{CelesteIo, Lookup, LookupRef, Node, NodeReadError, NonRleString, StringReadError},
    screen::{decode_screens, ScreensDecodeError},
    Screen,
};

#[derive(Debug)]
pub struct CelesteMap {
    name: String,
    pub(crate) unread: Node,
    fillers: Vec<Filler>,
    screens: Vec<Screen>,
}

impl CelesteMap {
    pub fn new(name: String) -> Self {
        CelesteMap {
            name,
            unread: Node::new("Map".into()),
            fillers: Vec::new(),
            screens: Vec::new(),
        }
    }

    pub fn read<R: Read>(mut reader: BufReader<R>) -> Result<Self, CelesteMapReadError> {
        <CelesteMap as CelesteIo>::read(&mut reader, None)
    }

    pub fn fillers(&self) -> &[Filler] {
        &self.fillers
    }

    pub fn fillers_mut(&mut self) -> &mut Vec<Filler> {
        &mut self.fillers
    }

    pub fn screens(&self) -> &[Screen] {
        &self.screens
    }

    pub fn screens_mut(&mut self) -> &mut Vec<Screen> {
        &mut self.screens
    }
}

#[derive(Error, Debug)]
pub enum CelesteMapReadError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("not expecting lookup")]
    GivenLookup,
    #[error("map header malformed")]
    MalformedHeader(StringReadError),
    #[error("map header is incorrect")]
    IncorrectHeader,
    #[error("map name malformed")]
    MapNameError(#[from] StringReadError),
    #[error("root node read error")]
    RootNodeError(#[from] NodeReadError),
    #[error("failed decoding fillers")]
    FillersDecodeError(#[from] FillersDecodeError),
    #[error("failed decoding screens")]
    ScreensDecodeError(#[from] ScreensDecodeError),
}

impl CelesteIo for CelesteMap {
    type Error = CelesteMapReadError;

    fn read<R: Read>(
        reader: &mut BufReader<R>,
        lookup: Option<LookupRef<'_>>,
    ) -> Result<Self, Self::Error> {
        if lookup.is_some() {
            return Err(CelesteMapReadError::GivenLookup);
        }

        if NonRleString::read(reader, None)
            .map_err(CelesteMapReadError::MalformedHeader)?
            .0
            != "CELESTE MAP"
        {
            return Err(CelesteMapReadError::IncorrectHeader);
        }

        let mut map = CelesteMap::new(NonRleString::read(reader, None)?.0);

        let lookup = Lookup::new({
            let count = u16::read(reader, lookup)? as usize;
            let mut lookup = Vec::with_capacity(count);
            for _ in 0..count {
                lookup.push(NonRleString::read(reader, None)?.0)
            }
            lookup
        });

        map.unread = Node::read(reader, Some(lookup.as_ref()))?;
        decode_fillers(&mut map)?;
        decode_screens(&mut map)?;

        Ok(map)
    }
}
