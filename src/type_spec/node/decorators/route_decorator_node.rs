use crate::type_spec::node::InterfaceDecorator;

#[derive(Debug)]
pub struct RouteDecoratorNode {
    pub path: String,
}

impl InterfaceDecorator for RouteDecoratorNode {}
