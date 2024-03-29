use std::fmt::Display;
use std::str::FromStr;

use yaml_rust::{yaml, Yaml};

use crate::openapi_parser::common::{check_unexpected_keys, get_value};
use crate::openapi_parser::node::data_model_node::data_model_node::DataModelNode;

#[derive(Debug, Clone)]
pub struct StringNode {
    pub title: Option<String>,
    pub string_enum: Option<Vec<String>>,
    #[allow(dead_code)]
    nullable: Option<bool>,
    #[allow(dead_code)]
    example: Option<String>,
    #[allow(dead_code)]
    description: Option<String>,
    #[allow(dead_code)]
    default: Option<String>,
    pub pattern: Option<String>,
    pub format: Option<StringFormat>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    #[allow(dead_code)]
    x_faker: Option<String>,
}

#[derive(Debug, Clone)]
pub enum StringFormat {
    Date,
    DateTime,
    Byte,
    Binary,
}

impl Display for StringFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Date => write!(f, "date"),
            Self::DateTime => write!(f, "date-time"),
            Self::Byte => write!(f, "byte"),
            Self::Binary => write!(f, "binary"),
        }
    }
}

impl FromStr for StringFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "date" => Ok(Self::Date),
            "date-time" => Ok(Self::DateTime),
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
