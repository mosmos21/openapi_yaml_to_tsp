use crate::openapi_parser::node::Operation;
use crate::type_spec::node::decorators::{Method, MethodDecoratorNode};

impl From<Operation> for MethodDecoratorNode {
    fn from(operation: Operation) -> Self {
        match operation {
            Operation::Get => MethodDecoratorNode::new(Method::Get),
            Operation::Post => MethodDecoratorNode::new(Method::Post),
            Operation::Put => MethodDecoratorNode::new(Method::Put),
            Operation::Delete => MethodDecoratorNode::new(Method::Delete),
            Operation::Patch => MethodDecoratorNode::new(Method::Patch),
        }
    }
}
