use std::fmt::Display;

use crate::type_spec::node::common::string_literal;

#[derive(Debug)]
pub struct EnumNode {
    pub title: String,
    pub items: Box<Vec<String>>,
}

impl Display for EnumNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .items
            .iter()
            .map(|item| string_literal(item))
            .collect::<Vec<String>>()
            .join(",\n");

        write!(f, "enum {} {{\n{}\n}}", self.title, items)
    }
}
