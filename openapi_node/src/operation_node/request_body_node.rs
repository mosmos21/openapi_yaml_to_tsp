use yaml_rust::Yaml;

use crate::common::YamlHash;
use crate::{ContentType, DataModelNode};

#[derive(Debug)]
pub struct RequestBodyNode {
    pub content_type: ContentType,
    pub schema: DataModelNode,
    pub examples: Box<Vec<Yaml>>,
}

impl TryFrom<&Yaml> for RequestBodyNode {
    type Error = &'static str;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        let hash = yaml
            .as_hash()
            .ok_or("[ReqeustBodyNode::try_from] expected a hash")?;
        if hash.len() != 1 {
            return Err("[ReqeustBody::try_from] expected a hash with a single key");
        }

        let (content_type, content) = hash.iter().next().unwrap();
        let content = YamlHash::new(
            content
                .as_hash()
                .ok_or("[ReqeustBodyNode::try_from] expected a hash")?,
        );

        let content_type = content_type
            .as_str()
            .and_then(|str| str.parse().ok())
            .ok_or("[ReqeustBody::try_from] expected a string")?;
        let schema = content
            .get_value("schema")
            .and_then(|yaml| DataModelNode::try_from(yaml).ok())
            .ok_or("[ReqeustBody::try_from] expected a schema")?;
        let examples = content.get_vec("examples").cloned().unwrap_or(Vec::new());

        Ok(Self {
            content_type,
            schema,
            examples: Box::new(examples),
        })
    }
}
