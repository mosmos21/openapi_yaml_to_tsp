use crate::type_spec::node::decorators::TypeSpecDecorator;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct NamespaceNode {
    pub decorators: Box<Vec<Box<dyn NameSpaceDecorator>>>,
    pub name: String,
}

impl Display for NamespaceNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decorators = self
            .decorators
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        if self.decorators.len() == 0 {
            write!(f, "namespace {} {{}}", self.name)
        } else {
            write!(f, "{}\nnamespace {} {{}}", decorators, self.name)
        }
    }
}

pub trait NameSpaceDecorator: TypeSpecDecorator {}
