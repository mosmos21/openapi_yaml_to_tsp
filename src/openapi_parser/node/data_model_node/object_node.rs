use crate::openapi_parser::common::{check_unexpected_keys, get_value};
use crate::openapi_parser::node::data_model_node::{build_data_model_node, DataModelNode};
use std::collections::HashSet;
use yaml_rust::{yaml, Yaml};

#[derive(Debug, Clone)]
pub struct ObjectNode {
    pub title: Option<String>,
    #[allow(dead_code)]
    pub properties: Box<Vec<PropertyNode>>,
    #[allow(dead_code)]
    pub nullable: Option<bool>,
    #[allow(dead_code)]
    pub description: Option<String>,
    #[allow(dead_code)]
    pub example: Option<yaml::Hash>,
}

#[derive(Debug, Clone)]
pub struct PropertyNode {
    #[allow(dead_code)]
    key: String,
    #[allow(dead_code)]
    value: DataModelNode,
    #[allow(dead_code)]
    required: bool,
}

const EXPECTED_KEYS: [&'static str; 8] = [
    "type",
    "title",
    "properties",
    "required",
    "description",
    "nullable",
    "x-examples",
    "example",
];

fn get_required(hash: &yaml::Hash) -> HashSet<String> {
    let required_key = &Yaml::String(String::from("required"));

    hash.get(required_key)
        .and_then(|v| {
            if let Yaml::Array(a) = v {
                Some(a)
            } else {
                None
            }
        })
        .unwrap_or(&yaml::Array::new())
        .iter()
        .map(|key| key.as_str().unwrap_or_default().to_string())
        .collect()
}

fn get_properties(hash: &yaml::Hash) -> Vec<PropertyNode> {
    let properties_key = &Yaml::String(String::from("properties"));

    let properties = hash.get(properties_key).and_then(|yaml| yaml.as_hash());
    if properties.is_none() {
        return vec![];
    }
    let properties = properties.unwrap();
    let required_keys = get_required(hash);

    let mut result = vec![];
    for (key, value) in properties.iter() {
        if let (Yaml::String(key), Yaml::Hash(property)) = (key, value) {
            if let Some(node) = build_data_model_node(property, Some(key.clone())) {
                result.push(PropertyNode {
                    key: key.clone(),
                    value: node,
                    required: required_keys.contains(key),
                });
            } else {
                dbg!(key, value);
                panic!("invalid property");
            }
        } else {
            dbg!(key, value);
            panic!("unexpected key: {:?}, value: {:?}", key, value);
        }
    }

    result
}

pub fn build_object_node(hash: &yaml::Hash, title: Option<String>) -> Option<DataModelNode> {
    if Some("object".to_string()) != get_value(hash, "type")
        && !hash.contains_key(&Yaml::String(String::from("properties")))
    {
        return None;
    }
    check_unexpected_keys(EXPECTED_KEYS.to_vec(), hash);

    let x_examples = hash
        .get(&Yaml::String(String::from("x-examples")))
        .and_then(|yaml| yaml.as_hash())
        .map(|hash| hash.clone());
    let example = hash
        .get(&Yaml::String(String::from("example")))
        .and_then(|yaml| yaml.as_hash())
        .map(|hash| hash.clone());
    if x_examples.is_some() && example.is_some() {
        panic!("x-examples and example cannot be used together");
    }

    Some(DataModelNode::Object(ObjectNode {
        title: get_value(hash, "title").or(title),
        properties: Box::new(get_properties(hash)),
        nullable: get_value(hash, "nullable"),
        description: get_value(hash, "description"),
        example: x_examples.or(example),
    }))
}
