use crate::compiler::CompilerEnv;
use crate::openapi_parser::node as openapi_node;
use crate::type_spec::node as type_spec_node;

pub fn build_namespace_node(
    info_node: &openapi_node::InfoNode,
    env: &CompilerEnv,
) -> type_spec_node::NamespaceNode {
    let service = type_spec_node::decorators::ServiceDecorator {
        title: info_node.title.clone(),
        version: info_node.version.clone(),
    };
    let contact = type_spec_node::decorators::ContactNode {
        email: Some(info_node.contact.email.clone()),
        name: Some(info_node.contact.name.clone()),
        url: Some(info_node.contact.url.clone()),
    };
    let info = type_spec_node::decorators::AdditionalInfoNode {
        contact: Some(contact),
        license: None,
        terms_of_service: Some(info_node.terms_of_service.clone()),
    };
    let decorators: Vec<Box<dyn type_spec_node::NameSpaceDecorator>> =
        vec![Box::new(service), Box::new(info)];

    type_spec_node::NamespaceNode {
        decorators: Box::new(decorators),
        name: env.namespace.clone(),
    }
}
