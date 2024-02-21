use crate::type_spec::node::decorators::{LibInfo, TypeSpecDecorator};
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
        let method = match self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Put => "put",
            Self::Delete => "delete",
            Self::Patch => "patch",
        };
        write!(f, "{}", method)
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

impl Display for MethodDecoratorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.method)
    }
}

impl LibInfo for MethodDecoratorNode {
    fn get_lib_name(&self) -> Option<&'static str> {
        Some("@typespec/http")
    }
    fn get_namespace(&self) -> Option<&'static str> {
        Some("TypeSpec.Http")
    }
}

impl TypeSpecDecorator for MethodDecoratorNode {}

impl OperationDecorator for MethodDecoratorNode {}
