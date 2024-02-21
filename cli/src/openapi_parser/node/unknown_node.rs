use crate::openapi_parser::node::OpenAPINode;
use std::path::PathBuf;
use yaml_rust::yaml;

pub fn parse_unknown_content(
    hash: yaml::Hash,
    path: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    let unknown_keys = hash
        .iter()
        .map(|(k, _)| k.as_str().unwrap())
        .collect::<Vec<_>>()
        .join(", ");
    dbg!(path, unknown_keys);

    return (
        Some(vec![OpenAPINode::Unknown(Box::new(hash))]),
        yaml::Hash::new(),
    );
}
