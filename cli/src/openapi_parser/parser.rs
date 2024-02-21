use std::path::PathBuf;

use yaml_rust::yaml;

use crate::openapi_parser::node::{
    parse_data_model_content, parse_data_models_content, parse_example_content, parse_info_content,
    parse_metadata_content, parse_operation_content, parse_parameters_content, parse_paths_content,
    parse_servers_content, parse_tags_content, parse_unknown_content, OpenAPIFileNode, OpenAPINode,
};
use crate::yaml_loader::YamlFile;

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
    let mut result = Vec::new();

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

fn parse_yaml_file(file: &YamlFile) -> OpenAPIFileNode {
    assert_eq!(file.content.len(), 1);

    let path = file.path.clone();
    let hash = file
        .content
        .get(0)
        .and_then(|c| c.as_hash())
        .expect("invalid yaml file")
        .to_owned();
    let content = parse_yaml_content(hash, &path);

    OpenAPIFileNode {
        path,
        contents: Box::new(content),
    }
}

pub fn parse_yaml_files(files: &Vec<YamlFile>) -> Vec<OpenAPIFileNode> {
    files.into_iter().map(parse_yaml_file).collect()
}
