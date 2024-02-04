use crate::openapi_parser::parser::OpenAPINode;
use std::path::PathBuf;
use yaml_rust::{yaml, Yaml};

#[derive(Debug)]
pub struct ParametersNode {}

fn build_parameters_node(_: &yaml::Array) -> Option<OpenAPINode> {
    Some(OpenAPINode::Parameters(ParametersNode {}))
}

pub fn parse_parameters_content(
    mut hash: yaml::Hash,
    _: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    let ref parameters_key = Yaml::String(String::from("parameters"));

    let node = hash
        .get(parameters_key)
        .and_then(|yaml| {
            if let Yaml::Array(a) = yaml {
                Some(a)
            } else {
                None
            }
        })
        .and_then(|parameters| build_parameters_node(parameters));

    if let Some(node) = node {
        hash.remove(parameters_key);
        (Some(vec![node]), hash)
    } else {
        (None, hash)
    }
}
