use crate::type_spec::node::type_spec_node::TypeSpecNode;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TypeSpecFileNode {
    #[allow(dead_code)]
    pub path: PathBuf,
    #[allow(dead_code)]
    pub contents: Box<Vec<TypeSpecNode>>,
}
