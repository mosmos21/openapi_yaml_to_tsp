use crate::openapi_parser::node::data_model_node::{build_data_model_node, DataModelNode};
use yaml_rust::yaml;

#[derive(Debug)]
pub struct ArrayNode {
    #[allow(dead_code)]
    items: Box<DataModelNode>,
}

pub fn build_array_node(hash: &yaml::Hash) -> Option<DataModelNode> {
    if let Some(items) = hash
        .get(&yaml::Yaml::String("items".to_string()))
        .and_then(|yaml| yaml.as_hash())
    {
        if let Some(node) = build_data_model_node(items, None) {
            Some(DataModelNode::Array(ArrayNode {
                items: Box::new(node),
            }))
        } else {
            dbg!(items);
            panic!("unexpected items: {:?}", items);
        }
    } else {
        None
    }
}
