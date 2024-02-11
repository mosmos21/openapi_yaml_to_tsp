use crate::openapi_parser::node::data_model_node::{build_data_model_node, DataModelNode};
use crate::openapi_parser::node::operation_node::content_type::ContentType;
use yaml_rust::{yaml, Yaml};

#[derive(Debug)]
pub struct RequestBodyNode {
    #[allow(dead_code)]
    content_type: ContentType,
    pub schema: DataModelNode,
    #[allow(dead_code)]
    examples: Box<Vec<Yaml>>,
}

pub fn build_request_body_node(hash: &yaml::Hash) -> Option<RequestBodyNode> {
    if let Some(content) = hash
        .get(&yaml::Yaml::String("content".to_string()))
        .and_then(|v| v.as_hash())
    {
        let content_keys = content.keys().collect::<Vec<_>>();
        assert_eq!(content_keys.len(), 1);

        let content_type = content_keys[0].as_str().expect("Invalid content type");
        let content = content
            .get(&yaml::Yaml::String(content_type.to_string()))
            .and_then(|v| v.as_hash())
            .expect("Invalid content");

        let schema = content
            .get(&yaml::Yaml::String("schema".to_string()))
            .and_then(|v| v.as_hash())
            .and_then(|h| build_data_model_node(h, None))
            .expect("Invalid schema");
        let examples = content
            .get(&Yaml::String("examples".to_string()))
            .and_then(|v| v.as_vec())
            .map(|v| v.clone())
            .unwrap_or(Vec::new());

        Some(RequestBodyNode {
            content_type: ContentType::from_str(content_type),
            schema,
            examples: Box::new(examples),
        })
    } else {
        None
    }
}
