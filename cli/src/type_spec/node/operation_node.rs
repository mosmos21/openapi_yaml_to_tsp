use crate::type_spec::node::decorators::TypeSpecDecorator;
use crate::type_spec::node::ModelContentNode;
use std::fmt::{Debug, Display};

pub trait OperationDecorator: TypeSpecDecorator {}

#[derive(Debug)]
pub struct OperationNode {
    pub name: String,
    pub decorators: Box<Vec<Box<dyn OperationDecorator>>>,
    pub parameters: Box<Vec<ParameterNode>>,
    pub responses: Box<Vec<ModelContentNode>>,
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
                    .join(" "),
            );
        }
        let parameters = self
            .parameters
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join(", ");
        let responses = self
            .responses
            .iter()
            .map(|r| format!("{}", r))
            .collect::<Vec<_>>()
            .join(" | ");
        result.push(format!("op {}({}): {};", self.name, parameters, responses));

        write!(f, "{}", result.join(" "))
    }
}

#[derive(Debug)]
pub struct ParameterNode {
    pub decorators: Box<Vec<Box<dyn ParameterDecorator>>>,
    pub name: String,
    pub type_model: ModelContentNode,
}

impl Display for ParameterNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = vec![];
        if self.decorators.len() > 0 {
            result.push(
                self.decorators
                    .iter()
                    .map(|d| format!("{}", d))
                    .collect::<Vec<_>>()
                    .join(" "),
            );
        }
        result.push(format!("{}: {}", self.name, self.type_model));

        write!(f, "{}", result.join(" "))
    }
}

pub trait ParameterDecorator: TypeSpecDecorator {}
