use std::fmt::Display;

use crate::type_spec::node::enum_node::EnumNode;
use crate::type_spec::node::*;

#[derive(Debug)]
pub enum TypeSpecNode {
    Imports(ImportLibNodes),
    NameSpace(NamespaceNode),
    Interface(InterfaceNode),
    Model(ModelNode),
    Enum(EnumNode),
    ModelAlias(ModelAliasNode),
}

impl Display for TypeSpecNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSpecNode::Imports(node) => write!(f, "{}", node),
            TypeSpecNode::NameSpace(node) => write!(f, "{}", node),
            TypeSpecNode::Interface(node) => write!(f, "{}", node),
            TypeSpecNode::Model(node) => write!(f, "{}", node),
            TypeSpecNode::Enum(node) => write!(f, "{}", node),
            TypeSpecNode::ModelAlias(node) => write!(f, "{}", node),
        }
    }
}
