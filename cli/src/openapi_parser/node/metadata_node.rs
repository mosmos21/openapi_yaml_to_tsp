use std::path::PathBuf;

use yaml_rust::yaml;

use crate::openapi_parser::common::get_value;
use crate::openapi_parser::node::OpenAPINode;

#[derive(Debug)]
pub struct MetadataNode {
    #[allow(dead_code)]
    openapi: String,
}

fn build_metadata_node(hash: &yaml::Hash) -> Option<MetadataNode> {
    if let Some(openapi) = get_value(hash, "openapi") {
        Some(MetadataNode { openapi })
    } else {
        None
    }
}

pub fn parse_metadata_content(
    mut hash: yaml::Hash,
    _: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    let node = build_metadata_node(&hash);

    if let Some(node) = node {
        hash.remove(&yaml::Yaml::String("openapi".to_string()));
        (Some(vec![OpenAPINode::Metadata(node)]), hash)
    } else {
        (None, hash)
    }
}
