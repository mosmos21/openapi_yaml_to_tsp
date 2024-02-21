use std::collections::HashSet;

use yaml_rust::{yaml, Yaml};

use crate::common::{check_unexpected_keys, YamlHash, YamlWithKey};
use crate::DataModelNode;

#[derive(Debug, Clone)]
pub struct ObjectNode {
    pub title: Option<String>,
    pub properties: Box<Vec<PropertyNode>>,
    pub nullable: Option<bool>,
    pub description: Option<String>,
    pub example: Option<yaml::Hash>,
}

#[derive(Debug, Clone)]
pub struct PropertyNode {
    pub key: String,
    pub value: DataModelNode,
    pub required: bool,
}

impl TryFrom<(&Yaml, &Yaml)> for PropertyNode {
    type Error = String;

    fn try_from((key, value): (&Yaml, &Yaml)) -> Result<Self, Self::Error> {
        let key = key
            .as_str()
            .ok_or("[PropertyNode::try_from] Expected string")?
            .to_string();
        let value = DataModelNode::try_from((value, Some(&key)))?;

        Ok(Self {
            key,
            value,
            required: false,
        })
    }
}

const EXPECTED_KEYS: [&'static str; 8] = [
    "type",
    "title",
    "properties",
    "required",
    "description",
    "nullable",
    "x-examples",
    "example",
];

impl<'a> TryFrom<YamlWithKey<'a>> for ObjectNode {
    type Error = String;

    fn try_from((yaml, key): YamlWithKey<'a>) -> Result<Self, Self::Error> {
        let raw_hash = yaml
            .as_hash()
            .ok_or("[ObjectNode::try_from] Expected hash")?;

        let hash = YamlHash::new(raw_hash);
        let type_name = hash.get_string("type");
        let properties = hash.get_hash("properties");

        if type_name != Some("object".to_string()) && properties.is_none() {
            return Err(format!(
                "[ObjectNode::try_from] Expected type to be object, found: {}",
                type_name.unwrap_or_default()
            ));
        }
        check_unexpected_keys(EXPECTED_KEYS.to_vec(), raw_hash)?;

        let mut properties = properties
            .expect("[ObjectNode::try_from] Expected properties")
            .iter()
            .map(PropertyNode::try_from)
            .collect::<Result<Vec<PropertyNode>, String>>()?;
        let requred_keys = hash
            .get_vec("required")
            .unwrap_or(&yaml::Array::new())
            .iter()
            .map(|key| key.as_str().unwrap_or_default().to_string())
            .collect::<HashSet<String>>();
        properties
            .iter_mut()
            .for_each(|p| p.required = requred_keys.contains(&p.key));

        let title = key.cloned().or(hash.get_string("title"));
        let nullable = hash.get_bool("nullable");
        let description = hash.get_string("description");
        let example = hash.get_hash("example");
        let x_example = hash.get_hash("x-examples");

        if example.is_some() && x_example.is_some() {
            return Err(
                "[ObjectNode::try_from] Expected only one of example or x-examples".to_string(),
            );
        }

        Ok(Self {
            title,
            properties: Box::new(properties),
            nullable,
            description,
            example: example.or(x_example).cloned(),
        })
    }
}
