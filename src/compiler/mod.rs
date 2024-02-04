use crate::{
    openapi_parser::{parse_yaml_file, OpenAPIFileNode, OpenAPINode},
    yaml_loader::load_yaml,
};
use std::collections::HashMap;
use std::path::PathBuf;

type FileNodeMap = HashMap<String, OpenAPIFileNode>;

// NOTE: tspのexampleがわからんので一旦nodeから消す
pub fn remove_examples(file_nodes: &mut FileNodeMap) {
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

pub fn compile(root_dir: &PathBuf) -> FileNodeMap {
    let yaml_files = load_yaml(&root_dir).flatten_files();

    let mut file_nodes = yaml_files
        .into_iter()
        .map(|file| parse_yaml_file(file))
        .map(|node| {
            (
                node.path
                    .to_str()
                    .expect("failed to convert path to str")
                    .to_string(),
                node,
            )
        })
        .collect::<HashMap<_, _>>();
    remove_examples(&mut file_nodes);

    file_nodes
}
