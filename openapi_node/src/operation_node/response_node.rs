use std::collections::HashMap;
use std::str::FromStr;

use yaml_rust::Yaml;

use crate::common::{YamlEntry, YamlHash};
use crate::{ContentType, DataModelNode};

#[derive(Debug)]
pub struct ResponseNode {
    pub status: ResponseStatus,
    pub description: Option<String>,
    pub content_type: Option<ContentType>,
    pub schema: Option<DataModelNode>,
    pub examples: Option<Box<HashMap<String, Yaml>>>,
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

impl ResponseStatus {
    pub fn get_code(&self) -> u16 {
        match self {
            ResponseStatus::OK => 200,
            ResponseStatus::Created => 201,
            ResponseStatus::Accepted => 202,
            ResponseStatus::NoContent => 204,
            ResponseStatus::MovedPermanently => 301,
            ResponseStatus::BadRequest => 400,
            ResponseStatus::Unauthorized => 401,
            ResponseStatus::Forbidden => 403,
            ResponseStatus::NotFound => 404,
            ResponseStatus::UnprocessableEntity => 422,
            ResponseStatus::InternalServerError => 500,
        }
    }
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

fn build_examples(yaml: &Yaml) -> Option<HashMap<String, Yaml>> {
    yaml.as_hash().and_then(|hash| {
        hash.iter()
            .map(|(key, value)| key.as_str().map(|key| (key.to_string(), value.clone())))
            .collect::<Option<HashMap<String, Yaml>>>()
    })
}

impl<'a> TryFrom<YamlEntry<'a>> for ResponseNode {
    type Error = String;

    fn try_from((key, value): YamlEntry<'a>) -> Result<Self, Self::Error> {
        let status = key
            .as_str()
            .ok_or("[ResponseNode::try_from] expected a key".to_string())
            .and_then(|str| ResponseStatus::from_str(&str))?;

        let hash = value
            .as_hash()
            .ok_or("[ResponseNode::try_from] Expected a hash")?;
        let hash = YamlHash::new(hash);

        let description = hash.get_string("description");
        let content = hash
            .get_hash("content")
            .expect("[ResponseNode::try_from] Expected a content");
        assert_eq!(content.len(), 1, "Expected exactly one content");

        let (content_type, content) = content.iter().next().unwrap();
        let content_type = content_type.as_str().and_then(|ty| ty.parse().ok());
        let content = YamlHash::new(
            content
                .as_hash()
                .ok_or("[ResponseNode::try_from] Expected a hash")?,
        );
        let schema = content
            .get_value("schema")
            .and_then(|schema| DataModelNode::try_from(schema).ok());
        let examples = content
            .get_value("examples")
            .and_then(build_examples)
            .map(Box::new);

        Ok(ResponseNode {
            status: status,
            description,
            content_type,
            schema,
            examples,
        })
    }
}
