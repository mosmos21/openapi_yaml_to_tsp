use std::str::FromStr;

use yaml_rust::Yaml;

use crate::common::{check_unexpected_keys, YamlHash};
use crate::DataModelNode;

#[derive(Debug, Clone)]
pub struct ParameterNode {
    pub name: String,
    pub position: ParameterPosition,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub schema: DataModelNode,
    pub parameter_enum: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub enum ParameterPosition {
    Query,
    Header,
    Path,
    Cookie,
}

impl FromStr for ParameterPosition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "query" => Ok(ParameterPosition::Query),
            "header" => Ok(ParameterPosition::Header),
            "path" => Ok(ParameterPosition::Path),
            "cookie" => Ok(ParameterPosition::Cookie),
            _ => Err(format!("Invalid parameter position: {}", s)),
        }
    }
}

const EXPECTED_KEYS: [&'static str; 9] = [
    "name",
    "in",
    "description",
    "schema",
    "enum",
    "required",
    "require",          // NOTE: typo 生成後のファイルには含まれていないので無視する
    "minimum",          // NOTE: 意味のないパラメータだが間違えて書かれている
    "exclusiveMinimum", // NOTE: 意味のないパラメータだが間違えて書かれている
];

fn try_build_parameter_enum_from_yaml(yaml: &Yaml) -> Result<Vec<String>, &'static str> {
    let mut result = vec![];
    let enum_value = yaml
        .as_vec()
        .ok_or("[try_build_parameter_enum_from_yaml] Expected hash")?;

    for item in enum_value {
        match item {
            Yaml::String(str) => result.push(str.clone()),
            Yaml::Integer(val) => result.push(val.clone().to_string()),
            _ => {
                return Err("[try_build_parameter_enum_from_yaml] Expected string or integer");
            }
        }
    }

    Ok(result)
}

impl TryFrom<&Yaml> for ParameterNode {
    type Error = String;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        let raw_hash = yaml
            .as_hash()
            .ok_or("[ParameterNode::try_from] Expected hash")?;
        let hash = YamlHash::new(raw_hash);
        check_unexpected_keys(EXPECTED_KEYS.to_vec(), raw_hash)?;

        let name = hash
            .get_string("name")
            .ok_or("[ParameterNode::try_from] Invalid parameter name")?;
        let position = hash
            .get_string("in")
            .ok_or("[ParameterNode::try_from] Invalid parameter position")?
            .parse()?;
        let description = hash.get_string("description");
        let required = hash.get_bool("required");
        let schema_name = format!("{name}_schema");
        let schema = hash
            .get_value("schema")
            .ok_or("[ParameterNode::try_from] Expected schema hash, but not found".to_string())
            .and_then(|yaml| DataModelNode::try_from((yaml, Some(&schema_name))))?;
        let parameter_enum = hash
            .get_value("enum")
            .map(|yaml| try_build_parameter_enum_from_yaml(yaml))
            .transpose()?;

        Ok(ParameterNode {
            name,
            position,
            description,
            required,
            schema,
            parameter_enum,
        })
    }
}
