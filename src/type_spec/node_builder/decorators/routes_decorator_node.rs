use crate::type_spec::node::decorators::RouteDecoratorNode;

impl From<String> for RouteDecoratorNode {
    fn from(path: String) -> Self {
        RouteDecoratorNode { path }
    }
}
