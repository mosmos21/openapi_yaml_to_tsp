use crate::type_spec::node::decorators::service_node::ServiceNode;

#[derive(Debug)]
pub struct NamespaceNode {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub decorators: Box<Vec<NamespaceDecorator>>,
}

#[derive(Debug)]
pub enum NamespaceDecorator {
    Service(ServiceNode),
}
