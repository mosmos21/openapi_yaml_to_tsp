use std::path::PathBuf;

use yaml_rust::yaml;

use crate::openapi_parser::common::get_value;
use crate::openapi_parser::node::OpenAPINode;

#[derive(Debug)]
pub struct ServerNode {
    #[allow(dead_code)]
    url: String,
    #[allow(dead_code)]
    description: String,
}

fn build_server(hash: &yaml::Hash) -> ServerNode {
    ServerNode {
        url: get_value(hash, "url").expect("url not found"),
        description: get_value(hash, "description").expect("description not found"),
    }
}

pub fn parse_servers_content(
    mut hash: yaml::Hash,
    _: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    if let Some(servers) = hash
        .get(&yaml::Yaml::String("servers".to_string()))
        .and_then(|y| y.as_vec())
    {
        let servers = servers
            .iter()
            .map(|yaml| yaml.as_hash())
            .collect::<Option<Vec<&yaml::Hash>>>()
            .expect("servers not found");
        let nodes = servers.into_iter().map(build_server).collect();

        hash.remove(&yaml::Yaml::String("servers".to_string()));
        (Some(vec![OpenAPINode::Servers(Box::new(nodes))]), hash)
    } else {
        (None, hash)
    }
}
