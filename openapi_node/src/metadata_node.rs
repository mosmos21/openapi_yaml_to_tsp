use yaml_rust::Yaml;

use crate::common::YamlHash;

#[derive(Debug)]
pub struct MetadataNode {
    pub openapi: String,
}

impl TryFrom<&Yaml> for MetadataNode {
    type Error = String;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        let hash = YamlHash::new(yaml.as_hash().ok_or("MetadataNode must be a hash")?);

        let openapi = hash
            .get_string("openapi")
            .ok_or("[MetadataNode::try_from] openapi is required")?;

        Ok(MetadataNode { openapi })
    }
}
