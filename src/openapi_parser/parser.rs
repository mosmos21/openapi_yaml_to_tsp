use crate::openapi_parser::node::{
    data_model_node::{parse_data_model_content, parse_data_models_content, DataModelNode},
    example_node::{parse_example_content, ExampleNode},
    info_node::{parse_info_content, InfoNode},
    metadata_node::{parse_metadata_content, MetadataNode},
    operation_node::{parse_operation_content, OperationNode},
    parameter_node::{parse_parameters_content, ParametersNode},
    path_node::{parse_paths_content, PathNode},
    server_node::{parse_servers_content, ServerNode},
    tag_node::{parse_tags_content, TagNode},
    unknown_node::parse_unknown_content,
};
use crate::yaml_loader::YamlFile;
use std::path::PathBuf;
use yaml_rust::yaml;

#[derive(Debug)]
pub struct OpenAPIFileNode {
    pub path: PathBuf,
    pub content: Box<Vec<OpenAPINode>>,
}

#[derive(Debug)]
pub enum OpenAPINode {
    Metadata(MetadataNode),
    Info(InfoNode),
    Servers(Box<Vec<ServerNode>>),
    Tags(Box<Vec<TagNode>>),
    Paths(Box<Vec<PathNode>>),
    Operation(OperationNode),
    DataModel(DataModelNode),
    Parameters(ParametersNode),
    Example(ExampleNode),
    Unknown(Box<yaml::Hash>),
}

fn parse_content(hash: yaml::Hash, path: &PathBuf) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    vec![
        parse_metadata_content,
        parse_info_content,
        parse_servers_content,
        parse_tags_content,
        parse_paths_content,
        parse_operation_content,
        parse_data_model_content,
        parse_data_models_content,
        parse_parameters_content,
        parse_example_content,
        parse_unknown_content,
    ]
    .iter()
    .fold((None, hash), |(node, hash), parser| {
        if node.is_some() {
            (node, hash)
        } else {
            parser(hash, path)
        }
    })
}

fn parse_yaml_content(mut hash: yaml::Hash, path: &PathBuf) -> Vec<OpenAPINode> {
    let mut result = vec![];

    while hash.len() > 0 {
        let len = hash.len();
        let (nodes, new_hash) = parse_content(hash, path);
        if let Some(nodes) = nodes {
            nodes.into_iter().for_each(|node| result.push(node));
        }
        if len == new_hash.len() {
            dbg!(path, &new_hash);
            panic!("infinite loop")
        }
        hash = new_hash;
    }

    result
}

pub fn parse_yaml_file(file: YamlFile) -> OpenAPIFileNode {
    assert_eq!(file.content.len(), 1);

    let path = file.path;
    let hash = file
        .content
        .get(0)
        .and_then(|c| c.as_hash())
        .expect("invalid yaml file")
        .to_owned();
    let content = parse_yaml_content(hash, &path);

    OpenAPIFileNode {
        path,
        content: Box::new(content),
    }
}
