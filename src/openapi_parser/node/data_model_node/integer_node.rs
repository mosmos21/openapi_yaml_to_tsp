use crate::openapi_parser::common::{check_unexpected_keys, get_value};
use crate::openapi_parser::node::data_model_node::DataModelNode;
use std::str::FromStr;
use yaml_rust::yaml;

#[derive(Debug, Clone)]
pub struct IntegerNode {
    #[allow(dead_code)]
    format: Option<IntegerFormat>,
    #[allow(dead_code)]
    description: Option<String>,
    #[allow(dead_code)]
    default: Option<String>,
    #[allow(dead_code)]
    minimum: Option<String>,
    #[allow(dead_code)]
    maximum: Option<String>,
    #[allow(dead_code)]
    exclusive_minimum: Option<bool>,
    #[allow(dead_code)]
    exclusive_maximum: Option<bool>,
    #[allow(dead_code)]
    nullable: Option<bool>,
    #[allow(dead_code)]
    integer_enum: Option<Vec<i64>>,
    #[allow(dead_code)]
    example: Option<String>,
    #[allow(dead_code)]
    x_faker: Option<String>,
}

#[derive(Debug, Clone)]
pub enum IntegerFormat {
    Int32,
    Int64,
}

impl FromStr for IntegerFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int32" => Ok(Self::Int32),
            "int64" => Ok(Self::Int64),
            _ => Err(format!("unexpected integer format: {}", s)),
        }
    }
}

const EXPECTED_KEYS: [&'static str; 12] = [
    "type",
    "format",
    "description",
    "default",
    "minimum",
    "maximum",
    "exclusiveMinimum",
    "exclusiveMaximum",
    "nullable",
    "enum",
    "example",
    "x-faker",
];

fn get_enum(hash: &yaml::Hash) -> Option<Vec<i64>> {
    let enum_values = hash
        .get(&yaml::Yaml::String("enum".to_string()))
        .and_then(|yaml| yaml.as_vec());

    if let Some(array) = enum_values {
        Some(
            array
                .into_iter()
                .map(|item| item.as_i64().expect("unexpected integer enum value"))
                .collect(),
        )
    } else {
        None
    }
}

pub fn build_integer_node(hash: &yaml::Hash) -> Option<DataModelNode> {
    if Some("integer".to_string()) != get_value(hash, "type") {
        return None;
    }
    check_unexpected_keys(EXPECTED_KEYS.to_vec(), hash);

    Some(DataModelNode::Integer(IntegerNode {
        format: get_value(hash, "format"),
        description: get_value(hash, "description"),
        default: get_value(hash, "default"),
        minimum: get_value(hash, "minimum"),
        maximum: get_value(hash, "maximum"),
        exclusive_minimum: get_value(hash, "exclusiveMinimum"),
        exclusive_maximum: get_value(hash, "exclusiveMaximum"),
        nullable: get_value(hash, "nullable"),
        integer_enum: get_enum(hash),
        example: get_value(hash, "example"),
        x_faker: get_value(hash, "x-faker"),
    }))
}
