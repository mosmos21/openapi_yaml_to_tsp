use std::fmt::Debug;

pub trait OperationDecorator: Debug {}

#[derive(Debug)]
pub struct OperationNode {
    pub name: String,
    pub decorators: Box<Vec<Box<dyn OperationDecorator>>>,
    pub parameters: Box<Vec<ParameterNode>>,
}

#[derive(Debug)]
pub struct ParameterNode {
    pub decorators: Box<Vec<Box<dyn ParameterDecorator>>>,
    pub name: String,
    pub type_name: String,
}

pub trait ParameterDecorator: Debug {}
