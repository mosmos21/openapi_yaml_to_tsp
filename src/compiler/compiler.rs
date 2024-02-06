use crate::{
    compiler::parse_postprocess,
    openapi_parser::{parse_yaml_files, DataModelNode, OpenAPIFileNode, OpenAPINode},
    type_spec::node_builder::build_from_openapi_file_node,
    yaml_loader::load_yaml,
};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub type OpenAPIFileNodeMap = HashMap<String, OpenAPIFileNode>;

type ComponentFilePathMap = HashMap<String, String>;

type PathFileMap = HashMap<String, String>;

fn write_log<T: Debug>(log_file_name: &str, content: &T) {
    let _ = fs::remove_file(log_file_name);

    let mut log_file = File::create(log_file_name).unwrap();

    log_file
        .write_all(format!("{:#?}", content).as_bytes())
        .unwrap();
}

fn build_openapi_file_node_map(file_nodes: Vec<OpenAPIFileNode>) -> OpenAPIFileNodeMap {
    file_nodes
        .into_iter()
        .map(|node| (node.path.to_str().unwrap().to_string(), node))
        .collect()
}

fn build_namespace(path: &PathBuf) -> String {
    let mut namespace = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap()
        .chars()
        .collect::<Vec<_>>();
    namespace[0] = namespace[0].to_ascii_uppercase();

    format!("{}Service", namespace.into_iter().collect::<String>())
}

fn build_path_file_map(file_nodes: &OpenAPIFileNodeMap) -> PathFileMap {
    let paths_node = file_nodes
        .values()
        .flat_map(|node| node.content.iter())
        .find(|node| {
            if let OpenAPINode::Paths(_) = node {
                true
            } else {
                false
            }
        })
        .and_then(|node| {
            if let OpenAPINode::Paths(paths) = node {
                Some(paths)
            } else {
                None
            }
        })
        .expect("failed to find paths node");

    paths_node
        .iter()
        .map(|node| {
            let file_name = node
                .ref_file_path
                .file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.to_owned())
                .expect("invalid file path");
            let path = node.path.clone();

            (file_name, path)
        })
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

#[derive(Debug)]
pub struct CompilerEnv {
    pub namespace: String,
    pub path_file_map: PathFileMap,
    pub component_file_path_map: ComponentFilePathMap,
}

pub fn compile(root_dir: &PathBuf) {
    let yaml_files = load_yaml(&root_dir).flatten_files();

    let mut openapi_file_node_map = build_openapi_file_node_map(parse_yaml_files(&yaml_files));
    parse_postprocess::remove_examples(&mut openapi_file_node_map);
    parse_postprocess::merge_parameter_nodes(&mut openapi_file_node_map);

    write_log("openapi_node.log", &openapi_file_node_map);

    let env = CompilerEnv {
        namespace: build_namespace(root_dir),
        component_file_path_map: build_component_file_path_map(&openapi_file_node_map),
        path_file_map: build_path_file_map(&openapi_file_node_map),
    };
    write_log("compiler_env.log", &env);

    let type_spec_file_nodes = openapi_file_node_map
        .values()
        .map(|node| build_from_openapi_file_node(node, &env))
        .collect::<Vec<_>>();
    write_log("type_spec_node.log", &type_spec_file_nodes);
}
