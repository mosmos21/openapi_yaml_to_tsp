use crate::openapi_parser::Operation;
use crate::type_spec::node::decorators::TypeSpecDecorator;
use std::fmt::{Debug, Display};

pub trait OperationDecorator: TypeSpecDecorator {}

#[derive(Debug)]
pub struct OperationNode {
    pub name: String,
    pub decorators: Box<Vec<Box<dyn OperationDecorator>>>,
    pub parameters: Box<Vec<ParameterNode>>,
}

impl Display for OperationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = vec![];
        if self.decorators.len() > 0 {
            result.push(
                self.decorators
                    .iter()
                    .map(|d| format!("{}", d))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        }
        result.push(format!("op {}(): void;", self.name));

        write!(f, "{}", result.join("\n"))
    }
}

#[derive(Debug)]
pub struct ParameterNode {
    pub decorators: Box<Vec<Box<dyn ParameterDecorator>>>,
    pub name: String,
    pub type_name: String,
}

pub trait ParameterDecorator: TypeSpecDecorator {}
