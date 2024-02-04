use crate::openapi_parser::common::get_value;
use crate::openapi_parser::parser::OpenAPINode;
use std::path::PathBuf;
use yaml_rust::yaml;

#[derive(Debug)]
pub struct TagNode {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    description: String,
    #[allow(dead_code)]
    external_docs: Option<ExternalDocs>,
}

#[derive(Debug)]
pub struct ExternalDocs {
    #[allow(dead_code)]
    url: String,
}

fn build_external_docs(hash: &yaml::Hash) -> Option<ExternalDocs> {
    if let Some(external_docs) = hash
        .get(&yaml::Yaml::String("externalDocs".to_string()))
        .and_then(|y| y.as_hash())
    {
        Some(ExternalDocs {
            url: get_value(external_docs, "url").expect("url not found"),
        })
    } else {
        None
    }
}

fn build_tag_node(hash: &yaml::Hash) -> TagNode {
    TagNode {
        name: get_value(hash, "name").expect("name not found"),
        description: get_value(hash, "description").expect("description not found"),
        external_docs: build_external_docs(hash),
    }
}

pub fn parse_tags_content(
    mut hash: yaml::Hash,
    _: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    if let Some(tags) = hash
        .get(&yaml::Yaml::String("tags".to_string()))
        .and_then(|y| y.as_vec())
    {
        let tags = tags
            .iter()
            .map(|yaml| yaml.as_hash())
            .collect::<Option<Vec<&yaml::Hash>>>()
            .expect("tags not found");
        let nodes = tags.into_iter().map(build_tag_node).collect();

        hash.remove(&yaml::Yaml::String("tags".to_string()));
        (Some(vec![OpenAPINode::Tags(Box::new(nodes))]), hash)
    } else {
        (None, hash)
    }
}
