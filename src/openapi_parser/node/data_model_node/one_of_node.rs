use crate::openapi_parser::node::data_model_node::data_model_node::{
    build_data_model_node, DataModelNode,
};
use yaml_rust::yaml;

#[derive(Debug)]
pub struct OneOfNode {
    #[allow(dead_code)]
    items: Vec<DataModelNode>,
}

pub fn build_one_of_node(hash: &yaml::Hash) -> Option<DataModelNode> {
    let items = hash
        .get(&yaml::Yaml::String("oneOf".to_string()))
        .and_then(|yaml| yaml.as_vec());
    if let Some(array) = items {
        let mut items = vec![];
        for item in array {
            if let yaml::Yaml::Hash(hash) = item {
                if let Some(node) = build_data_model_node(&"".to_string(), hash) {
                    items.push(node);
                } else {
                    panic!("unexpected oneOf item: {:?}", item);
                }
            } else {
                panic!("unexpected oneOf item: {:?}", item);
            }
        }
        Some(DataModelNode::OneOf(OneOfNode { items }))
    } else {
        None
    }
}
