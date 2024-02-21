use crate::openapi_parser::node::OpenAPINode;
use std::path::PathBuf;
use yaml_rust::{yaml, Yaml};

#[derive(Debug)]
pub struct ExampleNode {
    #[allow(dead_code)]
    yaml: Yaml,
}

fn build_example_node(yaml: &Yaml) -> Option<OpenAPINode> {
    Some(OpenAPINode::Example(ExampleNode { yaml: yaml.clone() }))
}

pub fn parse_example_content(
    hash: yaml::Hash,
    _: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    let ref value_key = Yaml::String(String::from("value"));

    let node = hash
        .get(value_key)
        .and_then(|value| build_example_node(value));

    if let Some(node) = node {
        (Some(vec![node]), yaml::Hash::new())
    } else {
        (None, hash)
    }
}
