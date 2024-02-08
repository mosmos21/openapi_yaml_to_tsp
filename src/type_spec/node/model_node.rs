use crate::type_spec::node::type_node::TypeNode;
use crate::type_spec::node::*;
use std::fmt::Display;

#[derive(Debug)]
pub struct ModelNode {
    pub name: String,
    pub record: RecordModelNode,
}

impl Display for ModelNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "model {} {{\n}}", self.name)
    }
}

#[derive(Debug)]
pub enum ModelContentNode {
    Record(RecordModelNode),
    Array(ArrayModelNode),
    Type(TypeNode),
    ModelRef(IdentifierNode),
    Union(Vec<ModelContentNode>),
}

#[derive(Debug)]
pub struct RecordModelNode {
    pub properties: Box<Vec<RecordPropertyNode>>,
}

#[derive(Debug)]
pub struct RecordPropertyNode {
    pub decorators: Box<Vec<Box<dyn RecordPropertyDecorator>>>,
    pub key: RecordPropertyKey,
    pub value: ModelContentNode,
    pub required: bool,
}

pub trait RecordPropertyDecorator: Debug {}

#[derive(Debug)]
pub enum RecordPropertyKey {
    Identifier(IdentifierNode),
    String(String),
}

#[derive(Debug)]
pub struct ArrayModelNode {
    items: Box<Vec<ModelContentNode>>,
}
