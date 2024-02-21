use std::str::FromStr;

use crate::common::{check_unexpected_keys, YamlHash, YamlWithKey};

#[derive(Debug, Clone, PartialEq)]
pub struct NumberNode {
    pub title: Option<String>,
    pub format: Option<NumberFormat>,
    pub description: Option<String>,
    pub default: Option<f64>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub nullable: Option<bool>,
    pub example: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberFormat {
    Float,
    Double,
}

impl FromStr for NumberFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "float" => Ok(Self::Float),
            "double" => Ok(Self::Double),
            _ => Err(format!(
                "[NumberFormat::from_str] Unexpected number format: {}",
                s
            )),
        }
    }
}

const EXPECTED_KEYS: [&'static str; 8] = [
    "type",
    "format",
    "description",
    "default",
    "minimum",
    "maximum",
    "nullable",
    "example",
];

impl<'a> TryFrom<YamlWithKey<'a>> for NumberNode {
    type Error = String;

    fn try_from((yaml, key): YamlWithKey<'a>) -> Result<Self, Self::Error> {
        let raw_hash = yaml
            .as_hash()
            .ok_or("[NumberNode::try_from] expected hash")?;
        let hash = YamlHash::new(raw_hash);
        let type_name = hash.get_string("type");

        if type_name != Some("number".to_string()) {
            return Err(format!(
                "[NumberNode::try_from] expected type: number, got: {:?}",
                type_name
            ));
        }

        check_unexpected_keys(EXPECTED_KEYS.to_vec(), raw_hash)?;

        let title = key.cloned().or(hash.get_string("title"));
        let format = hash
            .get_string("format")
            .map(|s| {
                s.parse()
                    .map_err(|e| format!("[NumberNode::try_from] {}", e))
            })
            .transpose()?;
        let description = hash.get_string("description");
        let default = hash.get_f64("default");
        let minimum = hash.get_f64("minimum");
        let maximum = hash.get_f64("maximum");
        let nullable = hash.get_bool("nullable");
        let example = hash.get_f64("example");

        Ok(Self {
            title,
            format,
            description,
            default,
            minimum,
            maximum,
            nullable,
            example,
        })
    }
}

#[cfg(test)]
mod test {
    use yaml_rust::YamlLoader;

    use super::*;

    #[test]
    fn test_number_node_try_from() {
        let yaml = r#"
            type: number
            format: float
            description: "This is a number"
            default: 1.0
            minimum: 0.0
            maximum: 10.0
            nullable: true
            example: 5.0
        "#;

        let yaml = YamlLoader::load_from_str(yaml).unwrap();
        let yaml = &yaml[0];

        let key = "key_value".to_string();
        let result = NumberNode::try_from((yaml, Some(&key)));
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.format, Some(NumberFormat::Float));
        assert_eq!(result.description, Some("This is a number".to_string()));
        assert_eq!(result.default, Some(1.0));
        assert_eq!(result.minimum, Some(0.0));
        assert_eq!(result.maximum, Some(10.0));
        assert_eq!(result.nullable, Some(true));
        assert_eq!(result.example, Some(5.0));
    }
}
