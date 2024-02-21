use yaml_rust::Yaml;

use crate::common::YamlHash;

#[derive(Debug)]
pub struct Contact {
    pub name: String,
    pub url: String,
    pub email: String,
}

impl TryFrom<&Yaml> for Contact {
    type Error = String;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        let hash = YamlHash::new(yaml.as_hash().ok_or("Contact must be a hash")?);

        let name = hash
            .get_string("name")
            .ok_or("[ContactNode::try_from] name is required")?;
        let url = hash
            .get_string("url")
            .ok_or("[ContactNode::try_from] url is required")?;
        let email = hash
            .get_string("email")
            .ok_or("[ContactNode::try_from] email is required")?;

        Ok(Contact { name, url, email })
    }
}

#[derive(Debug)]
pub struct InfoNode {
    pub title: String,
    pub version: String,
    pub contact: Contact,
    pub terms_of_service: String,
}

impl TryFrom<&Yaml> for InfoNode {
    type Error = String;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        let hash = YamlHash::new(yaml.as_hash().ok_or("InfoNode must be a hash")?);

        let title = hash
            .get_string("title")
            .ok_or("[InfoNode::try_from] title is required")?;
        let version = hash
            .get_string("version")
            .ok_or("[InfoNode::try_from] version is required")?;
        let contact = hash
            .get_value("contact")
            .ok_or("[InfoNode::try_from] contact is required".to_string())
            .and_then(Contact::try_from)?;
        let terms_of_service = hash
            .get_string("termsOfService")
            .ok_or("[InfoNode::try_from] termsOfService is required")?;

        Ok(InfoNode {
            title,
            version,
            contact,
            terms_of_service,
        })
    }
}
