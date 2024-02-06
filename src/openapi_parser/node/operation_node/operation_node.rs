use crate::openapi_parser::common::{check_unexpected_keys, get_value};
use crate::openapi_parser::node::operation_node::request_body_node::{
    build_request_body_node, RequestBodyNode,
};
use crate::openapi_parser::node::operation_node::response_node::{
    build_response_nodes, ResponseNode,
};
use crate::openapi_parser::node::operation_node::{build_parameters_node, ParameterNode};
use crate::openapi_parser::parser::OpenAPINode;
use std::path::PathBuf;
use yaml_rust::{yaml, Yaml};

#[derive(Debug)]
pub enum Operation {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl Operation {
    fn from_str(op: &str) -> Self {
        match op {
            "get" => Operation::Get,
            "post" => Operation::Post,
            "put" => Operation::Put,
            "delete" => Operation::Delete,
            "patch" => Operation::Patch,
            _ => panic!("[Operation::from_str] invalid operation"),
        }
    }
    fn as_yaml_str(&self) -> Yaml {
        let str = match self {
            Operation::Get => "get",
            Operation::Post => "post",
            Operation::Put => "put",
            Operation::Delete => "delete",
            Operation::Patch => "patch",
        };

        Yaml::String(String::from(str))
    }
}

#[derive(Debug)]
pub struct OperationNode {
    #[allow(dead_code)]
    pub op: Operation,
    #[allow(dead_code)]
    pub summary: Option<String>,
    #[allow(dead_code)]
    pub operation_id: Option<String>,
    #[allow(dead_code)]
    pub description: Option<String>,
    #[allow(dead_code)]
    pub tags: Box<Vec<String>>,
    #[allow(dead_code)]
    pub securities: Box<Vec<yaml::Hash>>,
    pub parameters: Box<Vec<ParameterNode>>,
    #[allow(dead_code)]
    pub request_body: Option<RequestBodyNode>,
    #[allow(dead_code)]
    pub responses: Box<Vec<ResponseNode>>,
}

const EXPECTED_KEYS: [&'static str; 8] = [
    "summary",
    "operationId",
    "description",
    "tags",
    "security",
    "parameters",
    "requestBody",
    "responses",
];

fn build_tags(hash: &yaml::Hash) -> Option<Vec<String>> {
    hash.get(&Yaml::String("tags".to_string()))
        .and_then(|v| v.as_vec())
        .and_then(|v| {
            v.iter()
                .map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
}

fn build_securities(hash: &yaml::Hash) -> Option<Vec<yaml::Hash>> {
    hash.get(&Yaml::String("security".to_string()))
        .and_then(|v| v.as_vec())
        .and_then(|v| {
            v.iter()
                .map(|v| v.as_hash().map(|h| h.clone()))
                .collect::<Option<Vec<_>>>()
        })
}

fn build_operation_node(hash: &yaml::Hash) -> Option<OpenAPINode> {
    let ops = [
        Yaml::String(String::from("get")),
        Yaml::String(String::from("post")),
        Yaml::String(String::from("put")),
        Yaml::String(String::from("delete")),
        Yaml::String(String::from("patch")),
    ];
    for op in ops {
        if let Some(operation) = hash.get(&op).and_then(|v| v.as_hash()) {
            check_unexpected_keys(EXPECTED_KEYS.to_vec(), operation);

            let parameters = operation
                .get(&Yaml::String("parameters".to_string()))
                .and_then(|v| v.as_vec())
                .and_then(|v| build_parameters_node(v));

            let request_body = operation
                .get(&Yaml::String("requestBody".to_string()))
                .and_then(|v| v.as_hash())
                .and_then(|h| build_request_body_node(h));

            let responses = operation
                .get(&Yaml::String("responses".to_string()))
                .and_then(|v| v.as_hash())
                .and_then(build_response_nodes);

            return Some(OpenAPINode::Operation(OperationNode {
                op: Operation::from_str(op.as_str().unwrap()),
                summary: get_value(operation, "summary"),
                operation_id: get_value(operation, "operationId"),
                description: get_value(operation, "description"),
                tags: Box::new(build_tags(operation).unwrap_or(Vec::new())),
                securities: Box::new(build_securities(operation).unwrap_or(Vec::new())),
                parameters: Box::new(parameters.unwrap_or(Vec::new())),
                request_body,
                responses: Box::new(responses.unwrap_or(Vec::new())),
            }));
        }
    }

    None
}

pub fn parse_operation_content(
    mut hash: yaml::Hash,
    _: &PathBuf,
) -> (Option<Vec<OpenAPINode>>, yaml::Hash) {
    if let Some(OpenAPINode::Operation(node)) = build_operation_node(&hash) {
        let _ = hash.remove(&node.op.as_yaml_str());

        (Some(vec![OpenAPINode::Operation(node)]), hash)
    } else {
        (None, hash)
    }
}
