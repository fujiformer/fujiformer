use std::{
    fmt::Display,
    io::{BufReader, Read},
};

use thiserror::Error;

use super::{
    lookup::{LookupError, LookupRef, LookupValue},
    raw::CelesteIo,
    value::ReadValueError,
};

use super::value::Value;

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    properties: Vec<(String, Value)>,
    children: Vec<Node>,
}

impl Node {
    pub fn new(name: String) -> Self {
        Node {
            name,
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn push_property(&mut self, key: String, value: Value) {
        self.properties.push((key, value));
    }

    pub fn push_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn take_child_with_name(&mut self, name: &str) -> Option<Node> {
        self.children
            .iter()
            .position(|x| x.name == name)
            .map(|i| self.children.swap_remove(i))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn properties(&self) -> &[(String, Value)] {
        &self.properties
    }

    pub fn take_properties(&mut self) -> Vec<(String, Value)> {
        std::mem::take(&mut self.properties)
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }

    pub fn take_children(&mut self) -> Vec<Node> {
        std::mem::take(&mut self.children)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", self.name)?;
        let mut inner_text = None;
        if !self.properties.is_empty() {
            for (key, value) in self.properties.iter() {
                if key == "innerText" {
                    inner_text = Some(value);
                } else {
                    write!(f, " {key}=\"{value}\"", key = key, value = value)?;
                }
            }
        }
        if self.children.is_empty() && inner_text.is_none() {
            write!(f, "/>")
        } else {
            write!(f, ">")?;
            if let Some(inner_text) = inner_text {
                write!(f, "{}", inner_text)?;
            }
            for child in self.children.iter() {
                write!(f, "{}", child)?;
            }
            write!(f, "</{}>", self.name)
        }
    }
}

#[derive(Error, Debug)]
pub enum NodeReadError {
    #[error("error reading bytes")]
    Io(#[from] std::io::Error),
    #[error("name of node failed lookup")]
    NameLookupError(LookupError),
    #[error("name of property key failed lookup")]
    PropertyKeyLookupError(LookupError),
    #[error("property value failed reading")]
    PropertyValueError(#[from] ReadValueError),
    #[error("error reading child")]
    ChildError(#[from] Box<NodeReadError>),
}

impl CelesteIo for Node {
    type Error = NodeReadError;

    fn read<R: Read>(
        reader: &mut BufReader<R>,
        lookup: Option<LookupRef<'_>>,
    ) -> Result<Self, Self::Error> {
        let mut node = Node::new(
            LookupValue::read(reader, lookup)
                .map_err(NodeReadError::NameLookupError)?
                .0,
        );

        for _ in 0..u8::read(reader, lookup)? {
            let key = LookupValue::read(reader, lookup)
                .map_err(NodeReadError::PropertyKeyLookupError)?
                .0;
            let value = Value::read(reader, lookup)?;
            node.push_property(key, value);
        }

        for _ in 0..u16::read(reader, lookup)? {
            node.push_child(Node::read(reader, lookup).map_err(Box::new)?);
        }

        Ok(node)
    }
}
