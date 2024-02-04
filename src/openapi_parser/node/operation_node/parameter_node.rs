use crate::openapi_parser::common::{check_unexpected_keys, get_value};
use crate::openapi_parser::node::data_model_node::{build_data_model_node, DataModelNode};
use crate::openapi_parser::parser::OpenAPINode;
use std::path::PathBuf;
use std::str::FromStr;
use yaml_rust::{yaml, Yaml};

#[derive(Debug)]
pub struct ParameterNode {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    position: ParameterPosition,
    #[allow(dead_code)]
    description: Option<String>,
    #[allow(dead_code)]
    required: Option<bool>,
    #[allow(dead_code)]
    schema: DataModelNode,
    #[allow(dead_code)]
    parameter_enum: Option<Vec<String>>,
}

#[derive(Debug)]
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

fn build_parameter_enum(array: &yaml::Array) -> Option<Vec<String>> {
    array
        .iter()
        .map(|e| match e {
            Yaml::String(s) => Some(s.clone()),
            Yaml::Integer(i) => Some(i.to_string()),
            _ => None,
        })
        .collect()
}

pub fn build_parameter_node(hash: &yaml::Hash) -> Option<ParameterNode> {
    check_unexpected_keys(EXPECTED_KEYS.to_vec(), hash);

    let name: String = get_value(hash, "name").expect("Invalid parameter name");
    let schema_name = format!("{name}_schema");
    let schema = hash
        .get(&yaml::Yaml::String("schema".to_string()))
        .and_then(|v| v.as_hash())
        .and_then(|h| build_data_model_node(h, None))
        .expect("Invalid parameter schema");

    let parameter_enum = hash
        .get(&yaml::Yaml::String("enum".to_string()))
        .and_then(|v| v.as_vec())
        .and_then(|v| build_parameter_enum(v));

    Some(ParameterNode {
        name: name.clone(),
        position: get_value(hash, "in").expect("Invalid parameter position"),
        description: get_value(hash, "description"),
        required: get_value(hash, "required"),
        schema,
        parameter_enum,
    })
}

pub fn build_parameters_node(array: &yaml::Array) -> Option<Vec<ParameterNode>> {
    array
        .iter()
        .map(|v| v.as_hash().and_then(|h| build_parameter_node(h)))
        .collect::<Option<Vec<_>>>()
}

pub fn parse_parameters_content(
    mut hash: yaml::Hash,
    _: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    if let Some(parameters) = hash
        .get(&yaml::Yaml::String("parameters".to_string()))
        .and_then(|v| v.as_vec())
    {
        let nodes = parameters
            .iter()
            .map(|v| v.as_hash().and_then(|h| build_parameter_node(h)))
            .collect::<Option<Vec<_>>>()
            .expect("Invalid parameters");

        hash.remove(&Yaml::String("parameters".to_string()));
        (Some(vec![OpenAPINode::Parameters(Box::new(nodes))]), hash)
    } else {
        (None, hash)
    }
}
