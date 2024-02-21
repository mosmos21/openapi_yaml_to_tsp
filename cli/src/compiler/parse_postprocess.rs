use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::compiler::FilePathObjectMap;
use crate::openapi_parser::node::*;

pub fn remove_examples(file_nodes: &mut Vec<OpenAPIFileNode>) {
    file_nodes.retain(|file_node| {
        file_node.contents.iter().all(|node| {
            if let OpenAPINode::Example(_) = node {
                false
            } else {
                true
            }
        })
    });
}

// =================================================================================================

fn list_parameter_nodes(file_node: &OpenAPIFileNode) -> Vec<ParameterNode> {
    file_node
        .contents
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
        .contents
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
    file_node.contents.retain(|node| {
        if let OpenAPINode::Parameters(_) = node {
            false
        } else {
            true
        }
    });
}

pub fn merge_parameter_nodes(file_nodes: &mut Vec<OpenAPIFileNode>) {
    file_nodes.iter_mut().for_each(|file_node| {
        let parameters = list_parameter_nodes(&file_node);
        insert_parameters_to_operation_node(file_node, parameters);
        delete_parameters_node(file_node);
    });
}

// =================================================================================================

fn replace_file_ref_to_compiler_ref_in_ref_node(
    ref_node: &mut RefNode,
    current_file_path: &PathBuf,
    map: &FilePathObjectMap,
) {
    if let RefNode::FileRef(file_ref) = ref_node {
        let base_dir = current_file_path
            .parent()
            .expect("Failed to get parent dir");
        let target_path = base_dir
            .join(Path::new(&file_ref.file_path))
            .canonicalize()
            .expect("Failed to canonicalize")
            .to_str()
            .map(|s| s.to_string())
            .expect("Failed to convert to string");
        let component_name = map
            .get(&target_path)
            .and_then(|node| node.title())
            .unwrap_or("AnonymousComponentRef".to_string());

        *ref_node = RefNode::ComponentRef(ComponentRefNode { component_name })
    }
}
fn replace_file_ref_to_compiler_ref_in_operation_node(
    node: &mut OperationNode,
    current_file_path: &PathBuf,
    map: &FilePathObjectMap,
) {
    node.parameters.iter_mut().for_each(|parameter| {
        replace_file_ref_to_compiler_ref_in_data_model_node(
            &mut parameter.schema,
            current_file_path,
            map,
        );
    });

    if let Some(request_body) = &mut node.request_body {
        replace_file_ref_to_compiler_ref_in_data_model_node(
            &mut request_body.schema,
            current_file_path,
            map,
        );
    }

    node.responses.iter_mut().for_each(|response| {
        if let Some(schema) = &mut response.schema {
            replace_file_ref_to_compiler_ref_in_data_model_node(schema, current_file_path, map);
        }
    });
}

fn replace_file_ref_to_compiler_ref_in_data_model_node(
    node: &mut DataModelNode,
    current_file_path: &PathBuf,
    map: &FilePathObjectMap,
) {
    match node {
        DataModelNode::Array(array) => {
            replace_file_ref_to_compiler_ref_in_data_model_node(
                &mut array.items,
                current_file_path,
                map,
            );
        }
        DataModelNode::Object(object) => {
            object.properties.iter_mut().for_each(|property_node| {
                replace_file_ref_to_compiler_ref_in_data_model_node(
                    &mut property_node.value,
                    current_file_path,
                    map,
                );
            });
        }
        DataModelNode::OneOf(one_of) => {
            one_of.items.iter_mut().for_each(|item| {
                replace_file_ref_to_compiler_ref_in_data_model_node(item, current_file_path, map);
            });
        }
        DataModelNode::AllOf(all_of) => {
            all_of.items.iter_mut().for_each(|item| {
                replace_file_ref_to_compiler_ref_in_data_model_node(item, current_file_path, map);
            });
        }
        DataModelNode::Ref(ref_node) => {
            replace_file_ref_to_compiler_ref_in_ref_node(ref_node, current_file_path, map);
        }
        _ => {}
    }
}

pub fn replace_file_ref_to_component_ref(
    file_nodes: &mut Vec<OpenAPIFileNode>,
    map: &FilePathObjectMap,
) {
    file_nodes.iter_mut().for_each(|file_node| {
        file_node.contents.iter_mut().for_each(|node| match node {
            OpenAPINode::Operation(node) => {
                replace_file_ref_to_compiler_ref_in_operation_node(node, &file_node.path, &map)
            }
            OpenAPINode::DataModel(data) => {
                replace_file_ref_to_compiler_ref_in_data_model_node(data, &file_node.path, &map)
            }
            _ => {}
        });
    });
}
