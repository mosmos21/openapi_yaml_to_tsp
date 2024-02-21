use super::build_import_lib_nodes_from_type_spec_node;
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
        contents: Box::new(vec![]),
    }
}

pub fn build_import_lib_nodes_from_namespace_node(
    namespace_node: &type_spec_node::NamespaceNode,
) -> Vec<type_spec_node::ImportLibNode> {
    let mut imports = vec![];

    for decorator in namespace_node.decorators.iter() {
        if let Some(import) = decorator.get_lib_name() {
            imports.push(type_spec_node::ImportLibNode::new(import.to_string()));
        }
    }

    imports
}

pub fn build_using_namespace_nodes_from_namespace_node(
    namespace_node: &type_spec_node::NamespaceNode,
) -> Vec<type_spec_node::UsingNamespaceNode> {
    let mut namespaces = vec![];

    for decorator in namespace_node.decorators.iter() {
        if let Some(namespace) = decorator.get_namespace() {
            namespaces.push(type_spec_node::UsingNamespaceNode::new(
                namespace.to_string(),
            ));
        }
    }

    namespaces
}
