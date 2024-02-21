use std::fmt::{Debug, Display};

use super::TypeSpecNode;
use crate::type_spec::node::decorators::TypeSpecDecorator;

#[derive(Debug)]
pub struct NamespaceNode {
    pub decorators: Box<Vec<Box<dyn NameSpaceDecorator>>>,
    pub name: String,
    pub contents: Box<Vec<TypeSpecNode>>,
}

impl Display for NamespaceNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = vec![];
        if self.decorators.len() > 0 {
            result.push(
                self.decorators
                    .iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }
        result.push(format!("namespace {} {{", self.name));
        if self.contents.len() > 0 {
            result.push(
                self.contents
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }
        result.push("}".to_string());

        write!(f, "{}", result.join("\n"))
    }
}

pub trait NameSpaceDecorator: TypeSpecDecorator {}
