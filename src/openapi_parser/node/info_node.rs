use crate::openapi_parser::common::get_value;
use crate::openapi_parser::node::OpenAPINode;
use std::path::PathBuf;
use yaml_rust::yaml;

#[derive(Debug)]
pub struct InfoNode {
    pub title: String,
    pub version: String,
    #[allow(dead_code)]
    contact: Contact,
    #[allow(dead_code)]
    terms_of_service: String,
}

#[derive(Debug)]
pub struct Contact {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    url: String,
    #[allow(dead_code)]
    email: String,
}

fn build_contact(hash: &yaml::Hash) -> Option<Contact> {
    if let Some(contact) = hash
        .get(&yaml::Yaml::String("contact".to_string()))
        .and_then(|y| y.as_hash())
    {
        Some(Contact {
            name: get_value(contact, "name").expect("name not found"),
            url: get_value(contact, "url").expect("url not found"),
            email: get_value(contact, "email").expect("email not found"),
        })
    } else {
        None
    }
}

fn build_info_node(hash: &yaml::Hash) -> Option<OpenAPINode> {
    if let Some(info) = hash
        .get(&yaml::Yaml::String("info".to_string()))
        .and_then(|y| y.as_hash())
    {
        Some(OpenAPINode::Info(InfoNode {
            title: get_value(info, "title").expect("title not found"),
            version: get_value(info, "version").expect("version not found"),
            contact: build_contact(info).expect("contact not found"),
            terms_of_service: get_value(info, "termsOfService")
                .expect("terms_of_service not found"),
        }))
    } else {
        None
    }
}

pub fn parse_info_content(
    mut hash: yaml::Hash,
    _: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    let node = build_info_node(&hash);

    if let Some(node) = node {
        hash.remove(&yaml::Yaml::String("info".to_string()));
        (Some(vec![node]), hash)
    } else {
        (None, hash)
    }
}
