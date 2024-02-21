use crate::common::{check_unexpected_keys, YamlHash, YamlWithKey};

#[derive(Debug, Clone)]
pub struct BooleanNode {
    pub title: Option<String>,
    pub default: Option<bool>,
    pub description: Option<String>,
    pub nullable: Option<bool>,
    pub example: Option<bool>,
}

const EXPECTED_KEYS: [&'static str; 5] = ["type", "description", "default", "nullable", "example"];

impl<'a> TryFrom<YamlWithKey<'a>> for BooleanNode {
    type Error = String;

    fn try_from((yaml, key): YamlWithKey<'a>) -> Result<Self, Self::Error> {
        let raw_hash = yaml
            .as_hash()
            .ok_or("[BooleanNode::try_from] Expected a hash")?;
        let hash = YamlHash::new(raw_hash);
        let type_value = hash.get_string("type");

        if type_value != Some("boolean".to_string()) {
            return Err(format!(
                "[BooleanNode::try_from] Expected boolean, got {:?}",
                type_value
            ));
        }
        check_unexpected_keys(EXPECTED_KEYS.to_vec(), raw_hash)?;

        let title = key.cloned().or(hash.get_string("title"));
        let default = hash.get_bool("default");
        let description = hash.get_string("description");
        let nullable = hash.get_bool("nullable");
        let example = hash.get_bool("example");

        Ok(Self {
            title,
            default,
            description,
            nullable,
            example,
        })
    }
}
