use crate::type_spec::node::TypeSpecFileNode;
use crate::{
    compiler::parse_postprocess, openapi_parser::node::*, openapi_parser::parse_yaml_files,
    type_spec::node_builder::build_type_spec_file_node, yaml_loader::load_yaml,
};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub type FilePathObjectMap = HashMap<String, DataModelNode>;

type PathFileMap = HashMap<String, String>;

fn write_log<T: Debug>(log_file_name: &str, content: &T) {
    let _ = fs::remove_file(log_file_name);

    let mut log_file = File::create(log_file_name).unwrap();

    log_file
        .write_all(format!("{:#?}", content).as_bytes())
        .unwrap();
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

fn build_path_file_map(file_nodes: &Vec<OpenAPIFileNode>) -> PathFileMap {
    let paths_node = file_nodes
        .iter()
        .flat_map(|node| node.contents.iter())
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

fn build_file_path_object_map(nodes: &Vec<OpenAPIFileNode>) -> FilePathObjectMap {
    let mut map = HashMap::new();
    for node in nodes.iter() {
        let path = node.path.to_str().unwrap().to_string();
        if node.contents.len() == 1 {
            if let Some(OpenAPINode::DataModel(data_model_node)) = node.contents.get(0) {
                map.insert(path, data_model_node.clone());
            }
        }
    }

    map
}

fn write_type_spec_file(file_node: &TypeSpecFileNode) {
    let file_name = file_node.path.to_str().unwrap();
    if Path::new(file_name).exists() {
        fs::remove_file(file_name).expect("failed to remove file");
    }

    let mut file = File::create(file_name).unwrap();
    file.write_all(file_node.to_string().as_bytes()).unwrap();
}

#[derive(Debug)]
pub struct CompilerEnv {
    pub namespace: String,
    pub path_file_map: PathFileMap,
    pub file_path_object_map: FilePathObjectMap,
}

pub fn compile(root_dir: &PathBuf) {
    let yaml_files = load_yaml(&root_dir).flatten_files();

    let mut openapi_file_nodes = parse_yaml_files(&yaml_files);
    parse_postprocess::remove_examples(&mut openapi_file_nodes);
    parse_postprocess::merge_parameter_nodes(&mut openapi_file_nodes);

    let env = CompilerEnv {
        namespace: build_namespace(root_dir),
        path_file_map: build_path_file_map(&openapi_file_nodes),
        file_path_object_map: build_file_path_object_map(&openapi_file_nodes),
    };
    write_log("compiler_env.log", &env);

    parse_postprocess::replace_file_ref_to_component_ref(
        &mut openapi_file_nodes,
        &env.file_path_object_map,
    );
    write_log("openapi_node.log", &openapi_file_nodes);

    let type_spec_file_nodes = openapi_file_nodes
        .into_iter()
        .map(|node| build_type_spec_file_node(node, &env))
        .collect::<Vec<_>>();
    write_log("type_spec_node.log", &type_spec_file_nodes);

    type_spec_file_nodes.iter().for_each(write_type_spec_file);
}
