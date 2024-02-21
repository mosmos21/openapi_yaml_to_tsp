use std::str::FromStr;

use yaml_rust::Yaml;

use crate::common::{check_unexpected_keys, YamlEntry, YamlHash};
use crate::{ParameterNode, RequestBodyNode, ResponseNode};

#[derive(Debug)]
pub enum Operation {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

#[derive(Debug)]
pub struct OperationNode {
    pub op: Operation,
    pub summary: Option<String>,
    pub operation_id: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Box<Vec<String>>>,
    pub securities: Option<Box<Vec<Yaml>>>,
    pub parameters: Option<Box<Vec<ParameterNode>>>,
    pub request_body: Option<RequestBodyNode>,
    pub responses: Box<Vec<ResponseNode>>,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "get" => Ok(Operation::Get),
            "post" => Ok(Operation::Post),
            "put" => Ok(Operation::Put),
            "delete" => Ok(Operation::Delete),
            "patch" => Ok(Operation::Patch),
            _ => Err(format!("[Operation::from_str] invalid operation: {}", op)),
        }
    }
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

impl<'a> TryFrom<YamlEntry<'a>> for OperationNode {
    type Error = String;

    fn try_from((key, value): YamlEntry<'a>) -> Result<Self, Self::Error> {
        let op = key
            .as_str()
            .ok_or("[OperationNode::try_from] key expected a string".to_string())
            .and_then(Operation::from_str)?;

        let raw_hash = value
            .as_hash()
            .ok_or("[OperationNode::try_from] invalid operation")?;
        let hash = YamlHash::new(raw_hash);
        check_unexpected_keys(EXPECTED_KEYS.to_vec(), raw_hash)?;

        let summary = hash.get_string("summary");
        let operation_id = hash.get_string("operationId");
        let description = hash.get_string("description");
        let tags = hash
            .get_vec("tags")
            .and_then(|tags| {
                tags.iter()
                    .map(|t| t.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .map(Box::new);
        let securities = hash
            .get_vec("security")
            .map(|v| v.iter().map(|y| y.clone()).collect())
            .map(Box::new);
        let parameters = hash
            .get_vec("parameters")
            .and_then(|params| {
                params
                    .iter()
                    .map(|p| ParameterNode::try_from(p))
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
            })
            .map(Box::new);
        let request_body = hash
            .get_value("requestBody")
            .and_then(|v| RequestBodyNode::try_from(v).ok());
        let responses = hash
            .get_hash("responses")
            .and_then(|responses| {
                responses
                    .iter()
                    .map(ResponseNode::try_from)
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
            })
            .map(Box::new)
            .unwrap_or_else(|| Box::new(vec![]));

        Ok(Self {
            op,
            summary,
            operation_id,
            description,
            tags,
            securities,
            parameters,
            request_body,
            responses,
        })
    }
}
