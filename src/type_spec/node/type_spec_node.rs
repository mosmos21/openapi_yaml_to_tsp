use crate::type_spec::node::{data::model_node::ModelNode, namespace_node::NamespaceNode};

#[derive(Debug)]
pub enum TypeSpecNode {
    Namespace(NamespaceNode),
    Model(ModelNode),
    Unknown,
}
