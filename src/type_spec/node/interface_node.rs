use crate::type_spec::node::decorators::TypeSpecDecorator;
use crate::type_spec::node::OperationNode;
use std::fmt::{Debug, Display};

pub trait InterfaceDecorator: TypeSpecDecorator {}

#[derive(Debug)]
pub struct InterfaceNode {
    pub name: String,
    pub decorators: Box<Vec<Box<dyn InterfaceDecorator>>>,
    pub operations: Box<Vec<OperationNode>>,
}

impl Display for InterfaceNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "interface {} {{\n}}\n", self.name)
    }
}
