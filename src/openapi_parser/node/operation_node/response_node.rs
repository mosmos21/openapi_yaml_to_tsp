use crate::openapi_parser::common::get_value;
use crate::openapi_parser::node::data_model_node::{build_data_model_node, DataModelNode};
use crate::openapi_parser::node::operation_node::content_type::ContentType;
use std::str::FromStr;
use yaml_rust::{yaml, Yaml};

#[derive(Debug)]
pub struct ResponseNode {
    #[allow(dead_code)]
    status: ResponseStatus,
    #[allow(dead_code)]
    description: Option<String>,
    #[allow(dead_code)]
    content_type: Option<ContentType>,
    #[allow(dead_code)]
    schema: Option<DataModelNode>,
}

#[derive(Debug)]
pub enum ResponseStatus {
    OK,
    Created,
    Accepted,
    NoContent,
    MovedPermanently,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    UnprocessableEntity,
    InternalServerError,
}

impl FromStr for ResponseStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "200" => Ok(ResponseStatus::OK),
            "201" => Ok(ResponseStatus::Created),
            "202" => Ok(ResponseStatus::Accepted),
            "204" => Ok(ResponseStatus::NoContent),
            "301" => Ok(ResponseStatus::MovedPermanently),
            "400" => Ok(ResponseStatus::BadRequest),
            "401" => Ok(ResponseStatus::Unauthorized),
            "403" => Ok(ResponseStatus::Forbidden),
            "404" => Ok(ResponseStatus::NotFound),
            "422" => Ok(ResponseStatus::UnprocessableEntity),
            "500" => Ok(ResponseStatus::InternalServerError),
            _ => Err(format!("Invalid response status: {}", s)),
        }
    }
}

fn build_response_node((status, yaml): (&Yaml, &Yaml)) -> Option<ResponseNode> {
    if let (Some(status), Some(hash)) = (status.as_str(), yaml.as_hash()) {
        let content = hash
            .get(&Yaml::String("content".to_string()))
            .and_then(|v| v.as_hash());
        let content_type = content
            .and_then(|c| c.keys().next())
            .and_then(|k| k.as_str())
            .map(ContentType::from_str);
        let content_inner = content_type
            .clone()
            .and_then(|c| content.and_then(|ctt| ctt.get(&Yaml::String(c.to_string()))))
            .and_then(|c| c.as_hash());
        let schema = content_inner
            .and_then(|c| c.get(&Yaml::String("schema".to_string())))
            .and_then(|y| y.as_hash())
            .and_then(|h| build_data_model_node(&"".to_string(), h));

        Some(ResponseNode {
            status: ResponseStatus::from_str(status).expect("Invalid response status"),
            description: get_value(hash, "description"),
            content_type,
            schema,
        })
    } else {
        None
    }
}

pub fn build_response_nodes(hash: &yaml::Hash) -> Option<Vec<ResponseNode>> {
    hash.iter().map(build_response_node).collect()
}
