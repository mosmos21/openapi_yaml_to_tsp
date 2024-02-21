use yaml_rust::Yaml;

use crate::common::YamlHash;

#[derive(Debug)]
pub struct ExternalDocs {
    pub url: String,
}

impl TryFrom<&Yaml> for ExternalDocs {
    type Error = String;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        let yaml = yaml
            .as_hash()
            .ok_or("[ExternalDocs::try_from] externalDocs must be a hash")
            .map(YamlHash::new)?;

        let url = yaml
            .get_string("url")
            .ok_or("[ExternalDocs::try_from] url is required")?;

        Ok(ExternalDocs { url })
    }
}

#[derive(Debug)]
pub struct TagNode {
    pub name: String,
    pub description: String,
    pub external_docs: Option<ExternalDocs>,
}

impl TryFrom<&Yaml> for TagNode {
    type Error = String;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        let yaml = yaml
            .as_hash()
            .ok_or("[TagNode::try_from] tag must be a hash")
            .map(YamlHash::new)?;

        let name = yaml
            .get_string("name")
            .ok_or("[TagNode::try_from] name is required")?;

        let description = yaml
            .get_string("description")
            .ok_or("[TagNode::try_from] description is required")?;

        let external_docs = yaml
            .get_value("externalDocs")
            .and_then(|yaml| ExternalDocs::try_from(yaml).ok());

        Ok(TagNode {
            name,
            description,
            external_docs,
        })
    }
}
