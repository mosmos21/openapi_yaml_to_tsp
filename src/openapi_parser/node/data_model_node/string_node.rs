use crate::openapi_parser::common::{check_unexpected_keys, get_value};
use crate::openapi_parser::node::data_model_node::data_model_node::DataModelNode;
use std::str::FromStr;
use yaml_rust::{yaml, Yaml};

#[derive(Debug, Clone)]
pub struct StringNode {
    #[allow(dead_code)]
    title: Option<String>,
    #[allow(dead_code)]
    string_enum: Option<Vec<String>>,
    #[allow(dead_code)]
    nullable: Option<bool>,
    #[allow(dead_code)]
    example: Option<String>,
    #[allow(dead_code)]
    description: Option<String>,
    #[allow(dead_code)]
    default: Option<String>,
    #[allow(dead_code)]
    pattern: Option<String>,
    #[allow(dead_code)]
    format: Option<StringFormat>,
    #[allow(dead_code)]
    min_length: Option<usize>,
    #[allow(dead_code)]
    max_length: Option<usize>,
    #[allow(dead_code)]
    x_faker: Option<String>,
}

#[derive(Debug, Clone)]
pub enum StringFormat {
    Date,
    DateTime,
    Password,
    Byte,
    Binary,
}

impl FromStr for StringFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "date" => Ok(Self::Date),
            "date-time" => Ok(Self::DateTime),
            "password" => Ok(Self::Password),
            "byte" => Ok(Self::Byte),
            "binary" => Ok(Self::Binary),
            _ => Err(format!("unexpected string format: {}", s)),
        }
    }
}

const EXPECTED_KEYS: [&'static str; 12] = [
    "type",
    "title",
    "enum",
    "nullable",
    "example",
    "description",
    "default",
    "pattern",
    "format",
    "minLength",
    "maxLength",
    "x-faker",
];

fn get_string_enum(hash: &yaml::Hash) -> Option<Vec<String>> {
    let enum_values = hash
        .get(&Yaml::String("enum".to_string()))
        .and_then(|yaml| yaml.as_vec());

    enum_values.map(|array| {
        array
            .iter()
            .map(|item| match item {
                Yaml::String(str) => str.clone(),
                Yaml::Integer(val) => val.clone().to_string(),
                _ => panic!("unexpected string enum value"),
            })
            .collect()
    })
}
pub fn build_string_node(hash: &yaml::Hash) -> Option<DataModelNode> {
    if Some("string".to_string()) != get_value(hash, "type") {
        return None;
    }
    check_unexpected_keys(EXPECTED_KEYS.to_vec(), hash);

    Some(DataModelNode::String(StringNode {
        title: get_value(hash, "title"),
        string_enum: get_string_enum(hash),
        nullable: get_value(hash, "nullable"),
        example: get_value(hash, "example"),
        description: get_value(hash, "description"),
        default: get_value(hash, "default"),
        pattern: get_value(hash, "pattern"),
        format: get_value(hash, "format"),
        min_length: get_value(hash, "minLength"),
        max_length: get_value(hash, "maxLength"),
        x_faker: get_value(hash, "x-faker"),
    }))
}
