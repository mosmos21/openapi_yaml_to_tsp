use crate::compiler::CompilerEnv;
use crate::openapi_parser::node as openapi_node;
use crate::type_spec::node as type_spec_node;
use crate::type_spec::node_builder::operation_node::{
    build_import_lib_nodes_from_operation_node, build_operation_node,
};
use std::path::PathBuf;
use std::thread::current;

pub fn build_interface_node(
    operations: &Vec<&openapi_node::OperationNode>,
    current_file_name: &str,
    env: &CompilerEnv,
) -> type_spec_node::InterfaceNode {
    let route = env
        .path_file_map
        .get(&current_file_name.replace(".tsp", ".yaml"))
        .expect("Failed to get route");
    let decorators: Vec<Box<dyn type_spec_node::InterfaceDecorator>> =
        vec![Box::new(type_spec_node::decorators::RouteDecoratorNode {
            path: route.to_owned(),
        })];
    let operations = operations
        .iter()
        .map(|op| build_operation_node(op))
        .collect::<Vec<_>>();

    type_spec_node::InterfaceNode {
        name: "Interface".to_string(),
        decorators: Box::new(decorators),
        operations: Box::new(operations),
    }
}

pub fn build_import_lib_nodes_from_interface_node(
    interface_node: &type_spec_node::InterfaceNode,
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
        imports.extend(build_import_lib_nodes_from_operation_node(operation))
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
