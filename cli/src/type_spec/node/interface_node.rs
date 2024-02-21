use std::fmt::{format, Debug, Display};

use crate::type_spec::node::decorators::TypeSpecDecorator;
use crate::type_spec::node::OperationNode;

pub trait InterfaceDecorator: TypeSpecDecorator {}

#[derive(Debug)]
pub struct InterfaceNode {
    pub name: String,
    pub decorators: Box<Vec<Box<dyn InterfaceDecorator>>>,
    pub operations: Box<Vec<OperationNode>>,
}

impl Display for InterfaceNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = vec![];
        if self.decorators.len() > 0 {
            result.push(
                self.decorators
                    .iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        }
        result.push(format!("interface {} {{", self.name));
        if self.operations.len() > 0 {
            result.push(
                self.operations
                    .iter()
                    .map(|op| format!("{}", op))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        }
        result.push("}".to_string());
        write!(f, "{}", result.join("\n"))
    }
}
