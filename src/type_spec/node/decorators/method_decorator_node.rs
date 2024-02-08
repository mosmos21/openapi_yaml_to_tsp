use crate::type_spec::node::OperationDecorator;
use std::fmt::Display;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().to_lowercase())
    }
}

#[derive(Debug)]
pub struct MethodDecoratorNode {
    method: Method,
}

impl MethodDecoratorNode {
    pub fn new(method: Method) -> Self {
        MethodDecoratorNode { method }
    }
}

impl OperationDecorator for MethodDecoratorNode {}
