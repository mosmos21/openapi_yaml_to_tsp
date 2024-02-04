use crate::openapi_parser::common::{check_unexpected_keys, get_value};
use crate::openapi_parser::node::data_model_node::DataModelNode;
use yaml_rust::yaml;

#[derive(Debug)]
pub struct BooleanNode {
    #[allow(dead_code)]
    default: Option<bool>,
    #[allow(dead_code)]
    description: Option<String>,
    #[allow(dead_code)]
    nullable: Option<bool>,
    #[allow(dead_code)]
    example: Option<bool>,
}

const EXPECTED_KEYS: [&'static str; 5] = ["type", "description", "default", "nullable", "example"];

pub fn build_boolean_node(hash: &yaml::Hash) -> Option<DataModelNode> {
    if Some("boolean".to_string()) != get_value(hash, "type") {
        return None;
    }
    check_unexpected_keys(EXPECTED_KEYS.to_vec(), hash);

    Some(DataModelNode::Boolean(BooleanNode {
        default: get_value(hash, "default"),
        description: get_value(hash, "description"),
        nullable: get_value(hash, "nullable"),
        example: get_value(hash, "example"),
    }))
}
