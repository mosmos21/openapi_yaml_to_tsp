use crate::compiler::OpenAPIFileNodeMap;
use crate::openapi_parser::{OpenAPIFileNode, OpenAPINode, ParameterNode};
use std::collections::HashSet;

pub fn remove_examples(file_nodes: &mut OpenAPIFileNodeMap) {
    let file_paths = file_nodes.keys().cloned().collect::<Vec<_>>();
    for path in file_paths {
        let node = file_nodes.get_mut(&path).unwrap();
        if node.content.iter().all(|node| {
            if let OpenAPINode::Example(_) = node {
                true
            } else {
                false
            }
        }) {
            file_nodes.remove(&path);
        }
    }
}

// =================================================================================================

fn list_parameter_nodes(file_node: &OpenAPIFileNode) -> Vec<ParameterNode> {
    file_node
        .content
        .iter()
        .filter_map(|node| {
            if let OpenAPINode::Parameters(parameters) = node {
                Some(parameters.clone())
            } else {
                None
            }
        })
        .flat_map(|parameters| parameters.into_iter().collect::<Vec<_>>())
        .collect()
}

fn insert_parameters_to_operation_node(
    file_node: &mut OpenAPIFileNode,
    parameters: Vec<ParameterNode>,
) {
    file_node
        .content
        .iter_mut()
        .filter_map(|node| {
            if let OpenAPINode::Operation(operation) = node {
                Some(operation)
            } else {
                None
            }
        })
        .for_each(|operation| {
            let parameter_names = operation
                .parameters
                .iter()
                .map(|p| p.name.clone())
                .collect::<HashSet<_>>();
            parameters
                .iter()
                .filter(|p| !parameter_names.contains(&p.name))
                .for_each(|p| {
                    operation.parameters.push(p.clone());
                });
        });
}

fn delete_parameters_node(file_node: &mut OpenAPIFileNode) {
    file_node.content.retain(|node| {
        if let OpenAPINode::Parameters(_) = node {
            false
        } else {
            true
        }
    });
}

pub fn merge_parameter_nodes(file_nodes: &mut OpenAPIFileNodeMap) {
    file_nodes.values_mut().for_each(|file_node| {
        let parameters = list_parameter_nodes(&file_node);
        insert_parameters_to_operation_node(file_node, parameters);
        delete_parameters_node(file_node);
    });
}
