use std::collections::HashMap;
use std::path::PathBuf;
use crate::{
    yaml_loader::load_yaml,
    openapi_parser::{parse_yaml_file, OpenAPIFileNode}
};

pub fn compile(root_dir: &PathBuf) -> HashMap<String, OpenAPIFileNode> {
    let yaml_files = load_yaml(&root_dir).flatten_files();

    let file_nodes = yaml_files
        .into_iter()
        .map(|file| parse_yaml_file(file))
        .map(|node| (
            node.path.to_str().expect("failed to convert path to str").to_string(),
            node
        ))
        .collect::<HashMap<_, _>>();

    file_nodes
}