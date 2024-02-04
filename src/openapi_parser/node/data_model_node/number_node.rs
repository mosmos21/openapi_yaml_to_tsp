use crate::openapi_parser::common::{check_unexpected_keys, get_value};
use crate::openapi_parser::node::data_model_node::DataModelNode;
use std::str::FromStr;
use yaml_rust::yaml;

#[derive(Debug)]
pub struct NumberNode {
    #[allow(dead_code)]
    format: Option<NumberFormat>,
    #[allow(dead_code)]
    description: Option<String>,
    #[allow(dead_code)]
    default: Option<String>,
    #[allow(dead_code)]
    minimum: Option<String>,
    #[allow(dead_code)]
    maximum: Option<String>,
    #[allow(dead_code)]
    nullable: Option<bool>,
    #[allow(dead_code)]
    example: Option<String>,
}

const EXPECTED_KEYS: [&'static str; 8] = [
    "type",
    "format",
    "description",
    "default",
    "minimum",
    "maximum",
    "nullable",
    "example",
];

#[derive(Debug)]
pub enum NumberFormat {
    Float,
    Double,
}

impl FromStr for NumberFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "float" => Ok(Self::Float),
            "double" => Ok(Self::Double),
            _ => Err(format!("unexpected number format: {}", s)),
        }
    }
}

pub fn build_number_node(hash: &yaml::Hash) -> Option<DataModelNode> {
    if Some("number".to_string()) != get_value(hash, "type") {
        return None;
    }
    check_unexpected_keys(EXPECTED_KEYS.to_vec(), hash);
    Some(DataModelNode::Number(NumberNode {
        format: get_value(hash, "format"),
        description: get_value(hash, "description"),
        default: get_value(hash, "default"),
        minimum: get_value(hash, "minimum"),
        maximum: get_value(hash, "maximum"),
        nullable: get_value(hash, "nullable"),
        example: get_value(hash, "example"),
    }))
}
