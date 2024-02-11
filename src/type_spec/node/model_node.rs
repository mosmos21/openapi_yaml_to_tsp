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
        write!(f, "model {} {}", self.name, self.record)
    }
}

#[derive(Debug)]
pub enum ModelContentNode {
    Record(RecordModelNode),
    Array(ArrayModelNode),
    Type(TypeNode),
    ModelRef(IdentifierNode),
    Union(Vec<ModelContentNode>),
    StringLiteral(String),
    Intersect(Vec<ModelContentNode>),
}

impl Display for ModelContentNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelContentNode::Record(record) => write!(f, "{}", record),
            ModelContentNode::Array(array) => write!(f, "{}", array),
            ModelContentNode::Type(t) => write!(f, "{}", t),
            ModelContentNode::ModelRef(id) => write!(f, "{}", id),
            ModelContentNode::Union(nodes) => {
                let nodes = nodes
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>();

                write!(f, "{}", nodes.join(" | "))
            }
            ModelContentNode::StringLiteral(s) => write!(f, "{}", s),
            ModelContentNode::Intersect(intersect) => {
                let nodes = intersect
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>();

                write!(f, "{}", nodes.join(" & "))
            }
        }
    }
}

#[derive(Debug)]
pub struct RecordModelNode {
    pub properties: Box<Vec<RecordPropertyNode>>,
}

impl Display for RecordModelNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let properties = self
            .properties
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{{\n{}\n}}", properties)
    }
}

#[derive(Debug)]
pub struct RecordPropertyNode {
    pub decorators: Box<Vec<Box<dyn RecordPropertyDecorator>>>,
    pub key: RecordPropertyKey,
    pub value: ModelContentNode,
    pub required: bool,
}

impl Display for RecordPropertyNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decorators = self
            .decorators
            .iter()
            .map(|d| format!("{}", d))
            .collect::<Vec<String>>()
            .join("\n");
        if self.decorators.is_empty() {
            write!(f, "{}: {};", &self.key, &self.value)
        } else {
            write!(f, "{}\n{}: {};", &decorators, &self.key, &self.value)
        }
    }
}

pub trait RecordPropertyDecorator: Display + Debug {}

#[derive(Debug)]
pub enum RecordPropertyKey {
    Identifier(IdentifierNode),
    String(String),
}

impl Display for RecordPropertyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordPropertyKey::Identifier(id) => write!(f, "{}", id),
            RecordPropertyKey::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Debug)]
pub struct ArrayModelNode {
    pub item_type: Box<ModelContentNode>,
}

impl Display for ArrayModelNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[]", self.item_type)
    }
}
