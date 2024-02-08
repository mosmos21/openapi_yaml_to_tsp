use crate::openapi_parser::node::*;
use std::path::PathBuf;
use yaml_rust::{yaml, Yaml};

#[derive(Debug, Clone)]
pub enum DataModelNode {
    Array(ArrayNode),
    Object(ObjectNode),
    OneOf(OneOfNode),
    String(StringNode),
    Integer(IntegerNode),
    Number(NumberNode),
    Boolean(BooleanNode),
    AllOf(AllOfNode),
    Ref(RefNode),
}

pub fn build_data_model_node(hash: &yaml::Hash, title: Option<String>) -> Option<DataModelNode> {
    if let Some(node) = build_object_node(hash, title) {
        return Some(node);
    }

    let builders = vec![
        build_string_node,
        build_integer_node,
        build_number_node,
        build_boolean_node,
        build_array_node,
        build_one_of_node,
        build_all_of_node,
        build_ref_node,
    ];
    for builder in builders {
        if let Some(node) = builder(hash) {
            return Some(node);
        }
    }

    None
}

pub fn parse_data_model_content(
    hash: yaml::Hash,
    path: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    let key = path
        .file_name()
        .and_then(|f| f.to_str())
        .expect("invalid file name")
        .replace(".yaml", "");

    if let Some(node) = build_data_model_node(&hash, Some(key)) {
        (Some(vec![OpenAPINode::DataModel(node)]), yaml::Hash::new())
    } else {
        (None, hash)
    }
}

pub fn parse_data_models_content(
    mut hash: yaml::Hash,
    path: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    let ref components_key = Yaml::String(String::from("components"));
    let ref schemas_key = Yaml::String(String::from("schemas"));

    let components = hash
        .get(components_key)
        .and_then(|c| c.as_hash())
        .and_then(|h| h.get(schemas_key))
        .and_then(|s| s.as_hash());

    if let Some(components) = components {
        let mut nodes = vec![];

        for (key, value) in components.iter() {
            if let (Yaml::String(key), Yaml::Hash(value)) = (key, value) {
                if let Some(node) = build_data_model_node(value, Some(key.clone())) {
                    nodes.push(node);
                } else {
                    dbg!(path, key, value);
                    panic!("invalid component");
                }
            } else {
                dbg!(path, key, value);
                panic!("invalid component");
            }
        }

        let _ = hash.remove(components_key);
        let nodes = nodes
            .into_iter()
            .map(|n| OpenAPINode::DataModel(n))
            .collect();
        return (Some(nodes), hash);
    }

    (None, hash)
}
