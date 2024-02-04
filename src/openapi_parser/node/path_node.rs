use crate::openapi_parser::common::get_value;
use crate::openapi_parser::parser::OpenAPINode;
use std::path::PathBuf;
use yaml_rust::{yaml, Yaml};

#[derive(Debug)]
pub struct PathNode {
    #[allow(dead_code)]
    path: String,
    #[allow(dead_code)]
    ref_file_path: PathBuf,
}

fn build_path_node((key, value): (&Yaml, &Yaml)) -> Option<PathNode> {
    if let (Some(key), Some(hash)) = (key.as_str(), value.as_hash()) {
        let file_path: String = get_value(hash, "$ref").expect("ref not found");

        Some(PathNode {
            path: key.to_string(),
            ref_file_path: PathBuf::from(file_path),
        })
    } else {
        None
    }
}

pub fn parse_paths_content(
    mut hash: yaml::Hash,
    _: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    if let Some(paths) = hash
        .get(&Yaml::String("paths".to_string()))
        .and_then(|y| y.as_hash())
    {
        let nodes = paths
            .iter()
            .map(build_path_node)
            .collect::<Option<Vec<_>>>()
            .expect("failed to parse paths");

        hash.remove(&yaml::Yaml::String("paths".to_string()));
        (Some(vec![OpenAPINode::Paths(Box::new(nodes))]), hash)
    } else {
        (None, hash)
    }
}
