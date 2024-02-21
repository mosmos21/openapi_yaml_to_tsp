use crate::type_spec::node::common::string_literal;
use crate::type_spec::node::decorators::{LibInfo, TypeSpecDecorator};
use crate::type_spec::node::InterfaceDecorator;
use std::fmt::Display;

#[derive(Debug)]
pub struct RouteDecoratorNode {
    pub path: String,
}

impl Display for RouteDecoratorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@route({})", string_literal(&self.path))
    }
}

impl LibInfo for RouteDecoratorNode {
    fn get_lib_name(&self) -> Option<&'static str> {
        Some("@typespec/http")
    }
    fn get_namespace(&self) -> Option<&'static str> {
        Some("TypeSpec.Http")
    }
}

impl TypeSpecDecorator for RouteDecoratorNode {}

impl InterfaceDecorator for RouteDecoratorNode {}
