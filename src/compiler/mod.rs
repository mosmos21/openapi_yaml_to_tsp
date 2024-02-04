use crate::{
    openapi_parser::{parse_yaml_files, DataModelNode, OpenAPIFileNode, OpenAPINode},
    yaml_loader::load_yaml,
};
use std::collections::HashMap;
use std::path::PathBuf;

type OpenAPIFileNodeMap = HashMap<String, OpenAPIFileNode>;

type ComponentFilePathMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Compiler {
    component_file_path_map: ComponentFilePathMap,
}

// NOTE: tspのexampleがわからんので一旦nodeから消す
fn remove_examples(file_nodes: &mut OpenAPIFileNodeMap) {
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

fn build_openapi_file_node_map(file_nodes: Vec<OpenAPIFileNode>) -> OpenAPIFileNodeMap {
    file_nodes
        .into_iter()
        .map(|node| (node.path.to_str().unwrap().to_string(), node))
        .collect()
}

fn build_component_file_path_map(map: &OpenAPIFileNodeMap) -> ComponentFilePathMap {
    let mut component_file_path_map = HashMap::new();
    for (path, node) in map.iter() {
        for n in node.content.iter() {
            if let OpenAPINode::DataModel(DataModelNode::Object(object)) = n {
                if let Some(title) = &object.title {
                    component_file_path_map.insert(title.clone(), path.clone());
                }
            }
        }
    }

    component_file_path_map
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            component_file_path_map: HashMap::new(),
        }
    }
    pub fn compile(&mut self, root_dir: &PathBuf) -> OpenAPIFileNodeMap {
        let yaml_files = load_yaml(&root_dir).flatten_files();

        let mut openapi_file_node_map = build_openapi_file_node_map(parse_yaml_files(&yaml_files));
        remove_examples(&mut openapi_file_node_map);
        self.component_file_path_map = build_component_file_path_map(&openapi_file_node_map);
        dbg!(&self.component_file_path_map);

        openapi_file_node_map
    }
}
