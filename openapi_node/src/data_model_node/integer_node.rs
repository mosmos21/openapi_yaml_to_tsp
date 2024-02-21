use std::str::FromStr;

use yaml_rust::Yaml;

use crate::common::{check_unexpected_keys, YamlHash, YamlWithKey};

#[derive(Debug, Clone, PartialEq)]
pub struct IntegerNode {
    pub title: Option<String>,
    pub format: Option<IntegerFormat>,
    pub description: Option<String>,
    pub default: Option<i64>,
    pub minimum: Option<i64>,
    pub maximum: Option<i64>,
    pub exclusive_minimum: Option<bool>,
    pub exclusive_maximum: Option<bool>,
    pub nullable: Option<bool>,
    pub integer_enum: Option<Vec<i64>>,
    pub example: Option<i64>,
    pub x_faker: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
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
            _ => Err(format!(
                "[IntegerFormat::from_str] Unexpected integer format: {}",
                s
            )),
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

fn try_build_integer_enum_from_yaml(yaml: &Yaml) -> Result<Vec<i64>, &'static str> {
    let mut result = vec![];
    let enum_value = yaml
        .as_vec()
        .ok_or("[try_build_integer_enum_from_yaml] Expected hash")?;

    for item in enum_value {
        match item {
            Yaml::Integer(str) => result.push(str.clone()),
            _ => {
                return Err("[try_build_integer_enum_from_yaml] Expected string or integer");
            }
        }
    }

    Ok(result)
}

impl<'a> TryFrom<YamlWithKey<'a>> for IntegerNode {
    type Error = String;

    fn try_from((yaml, key): YamlWithKey<'a>) -> Result<Self, Self::Error> {
        let raw_hash = yaml
            .as_hash()
            .ok_or("[IntegerNode::try_from] Expected hash")?;
        let hash = YamlHash::new(raw_hash);

        let type_value = hash.get_string("type");
        if type_value != Some("integer".to_string()) {
            return Err(format!(
                "[IntegerNode::try_from] Expected type: integer, got: {:?}",
                type_value
            ));
        }

        check_unexpected_keys(EXPECTED_KEYS.to_vec(), raw_hash)?;

        let format = hash.get_string("format").map(|s| {
            s.parse()
                .expect("[IntegerNode::try_from] Expected integer format")
        });

        let title = key.cloned().or(hash.get_string("title"));
        let description = hash.get_string("description");
        let default = hash.get_i64("default");
        let minimum = hash.get_i64("minimum");
        let maximum = hash.get_i64("maximum");
        let exclusive_minimum = hash.get_bool("exclusiveMinimum");
        let exclusive_maximum = hash.get_bool("exclusiveMaximum");
        let nullable = hash.get_bool("nullable");
        let integer_enum = hash
            .get_value("enum")
            .map(|yaml| try_build_integer_enum_from_yaml(yaml))
            .transpose()?;
        let example = hash.get_i64("example");
        let x_faker = hash.get_string("x-faker");

        Ok(Self {
            title,
            format,
            description,
            default,
            minimum,
            maximum,
            exclusive_minimum,
            exclusive_maximum,
            nullable,
            integer_enum,
            example,
            x_faker,
        })
    }
}
