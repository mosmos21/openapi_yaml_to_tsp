use yaml_rust::Yaml;

use crate::common::YamlHash;

#[derive(Debug)]
pub struct ServerNode {
    pub url: String,
    pub description: String,
}

impl TryFrom<&Yaml> for ServerNode {
    type Error = String;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        let hash = yaml
            .as_hash()
            .ok_or("[ServerNode::try_from] server must be a hash")
            .map(YamlHash::new)?;

        let url = hash
            .get_string("url")
            .ok_or("[ServerNode::try_from] url is required")?;
        let description = hash
            .get_string("description")
            .ok_or("[ServerNode::try_from] description is required")?;

        Ok(ServerNode { url, description })
    }
}
