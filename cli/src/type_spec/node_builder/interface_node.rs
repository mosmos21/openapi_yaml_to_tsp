use std::path::PathBuf;

use regex::Regex;

use crate::compiler::CompilerEnv;
use crate::openapi_parser::node as openapi_node;
use crate::type_spec::node as type_spec_node;
use crate::type_spec::node_builder::operation_node::{
    build_import_lib_nodes_from_operation_node, build_operation_node,
};

fn to_pascal_case(s: &str) -> String {
    let s = s.replace("{", "").replace("}", "");
    let re = Regex::new(r"(^((\w))|(_\w))").unwrap();
    re.replace_all(s.as_str(), |caps: &regex::Captures| {
        let s = &caps[1];
        if s.starts_with("_") {
            s[1..].to_uppercase()
        } else {
            s.to_uppercase()
        }
    })
    .to_string()
}

pub fn build_wrapped_interface_node(
    operations: &Vec<&openapi_node::OperationNode>,
    current_file_name: &str,
    env: &CompilerEnv,
) -> type_spec_node::NamespaceNode {
    let route = env
        .path_file_map
        .get(&current_file_name.replace(".tsp", ".yaml"))
        .expect("Failed to get route");
    let interface_name = route.as_str()[1..]
        .split("/")
        .map(|s| to_pascal_case(s))
        .collect::<Vec<_>>()
        .join("");
    let decorators: Vec<Box<dyn type_spec_node::InterfaceDecorator>> =
        vec![Box::new(type_spec_node::decorators::RouteDecoratorNode {
            path: route.to_owned(),
        })];
    let operations = operations
        .iter()
        .map(|op| build_operation_node(op))
        .collect::<Vec<_>>();

    let interface = type_spec_node::InterfaceNode {
        name: interface_name,
        decorators: Box::new(decorators),
        operations: Box::new(operations),
    };

    type_spec_node::NamespaceNode {
        decorators: Box::new(vec![]),
        name: env.namespace.clone(),
        contents: Box::new(vec![type_spec_node::TypeSpecNode::Interface(interface)]),
    }
}

pub fn build_import_lib_nodes_from_interface_node(
    interface_node: &type_spec_node::InterfaceNode,
    current_file_path: &PathBuf,
    env: &CompilerEnv,
) -> Vec<type_spec_node::ImportLibNode> {
    let mut imports = vec![];

    imports.extend(
        interface_node
            .decorators
            .iter()
            .filter_map(|node| node.get_lib_name())
            .map(type_spec_node::ImportLibNode::from)
            .collect::<Vec<_>>(),
    );

    interface_node.operations.iter().for_each(|operation| {
        imports.extend(build_import_lib_nodes_from_operation_node(
            operation,
            current_file_path,
            env,
        ))
    });

    imports
}

pub fn build_using_namespace_nodes_from_interface_node(
    interface_node: &type_spec_node::InterfaceNode,
) -> Vec<type_spec_node::UsingNamespaceNode> {
    let mut namespaces = vec![];

    for decorator in interface_node.decorators.iter() {
        if let Some(namespace) = decorator.get_namespace() {
            namespaces.push(type_spec_node::UsingNamespaceNode::new(
                namespace.to_string(),
            ));
        }
    }

    namespaces
}
