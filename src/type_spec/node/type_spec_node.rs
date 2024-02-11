use crate::type_spec::node::*;
use std::fmt::Display;

#[derive(Debug)]
pub enum TypeSpecNode {
    NameSpace(NamespaceNode),
    Interface(InterfaceNode),
    Model(ModelNode),
}

impl Display for TypeSpecNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSpecNode::NameSpace(node) => write!(f, "{}", node),
            TypeSpecNode::Interface(node) => write!(f, "{}", node),
            TypeSpecNode::Model(node) => write!(f, "{}", node),
        }
    }
}
