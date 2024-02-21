use std::str::FromStr;

use yaml_rust::Yaml;

use crate::common::{check_unexpected_keys, YamlHash, YamlWithKey};

#[derive(Debug, Clone)]
pub struct StringNode {
    pub title: Option<String>,
    pub string_enum: Option<Vec<String>>,
    pub nullable: Option<bool>,
    pub example: Option<String>,
    pub description: Option<String>,
    pub default: Option<String>,
    pub pattern: Option<String>,
    pub format: Option<StringFormat>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub x_faker: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringFormat {
    Date,
    DateTime,
    Byte,
    Binary,
}

impl FromStr for StringFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "date" => Ok(Self::Date),
            "date-time" => Ok(Self::DateTime),
            "byte" => Ok(Self::Byte),
            "binary" => Ok(Self::Binary),
            _ => Err(format!("unexpected string format: {}", s)),
        }
    }
}

const EXPECTED_KEYS: [&'static str; 12] = [
    "type",
    "title",
    "enum",
    "nullable",
    "example",
    "description",
    "default",
    "pattern",
    "format",
    "minLength",
    "maxLength",
    "x-faker",
];

fn try_build_string_enum_from_yaml(yaml: &Yaml) -> Result<Vec<String>, &'static str> {
    let mut result = vec![];
    let enum_value = yaml
        .as_vec()
        .ok_or("[try_build_string_enum_from_yaml] Expected hash")?;

    for item in enum_value {
        match item {
            Yaml::String(str) => result.push(str.clone()),
            Yaml::Integer(val) => result.push(val.clone().to_string()),
            _ => {
                return Err("[try_build_string_enum_from_yaml] Expected string or integer");
            }
        }
    }

    Ok(result)
}

impl<'a> TryFrom<YamlWithKey<'a>> for StringNode {
    type Error = String;

    fn try_from((yaml, key): YamlWithKey<'a>) -> Result<Self, Self::Error> {
        let raw_hash = yaml
            .as_hash()
            .ok_or("[StringNode::try_from] Expected hash")?;
        let hash = YamlHash::new(raw_hash);

        let type_value = hash.get_string("type");
        if type_value != Some("string".to_string()) {
            return Err("[StringNode::try_from] Expected type: string")?;
        }

        check_unexpected_keys(EXPECTED_KEYS.to_vec(), raw_hash)?;

        let title = key.cloned().or(hash.get_string("title"));
        let string_enum = hash
            .get_value("enum")
            .map(|v| try_build_string_enum_from_yaml(v))
            .transpose()?;
        let nullable = hash.get_bool("nullable");
        let example = hash.get_string("example");
        let description = hash.get_string("description").map(|s| s.to_string());
        let default = hash.get_string("default");
        let pattern = hash.get_string("pattern");
        let format = hash
            .get_string("format")
            .map(|s| {
                s.parse()
                    .map_err(|e| format!("[StringNode::try_from] {}", e))
            })
            .transpose()?;
        let min_length = hash.get_i64("minLength").map(|i| i as usize);
        let max_length = hash.get_i64("maxLength").map(|i| i as usize);
        let x_faker = hash.get_string("x-faker");

        Ok(Self {
            title,
            string_enum,
            nullable,
            example,
            description,
            default,
            pattern,
            format,
            min_length,
            max_length,
            x_faker,
        })
    }
}

#[cfg(test)]
mod test {
    use yaml_rust::YamlLoader;

    use super::*;

    #[test]
    fn test_string_node_try_from_yaml() {
        let yaml = r#"
            type: string
            enum:
              - test1
              - test2
            nullable: true
            example: example_value
            description: description_value
            default: default_value
            pattern: pattern_value
            format: date
            minLength: 1
            maxLength: 10
            x-faker: test
        "#;

        let yaml = YamlLoader::load_from_str(yaml).unwrap();
        let yaml = &yaml[0];
        let key = "key_value".to_string();
        let string_node = StringNode::try_from((yaml, Some(&key))).unwrap();

        assert_eq!(string_node.title, Some("title_value".to_string()));
        assert_eq!(
            string_node.string_enum,
            Some(vec!["test1".to_string(), "test2".to_string()])
        );
        assert_eq!(string_node.nullable, Some(true));
        assert_eq!(string_node.example, Some("example_value".to_string()));
        assert_eq!(
            string_node.description,
            Some("description_value".to_string())
        );
        assert_eq!(string_node.default, Some("default_value".to_string()));
        assert_eq!(string_node.pattern, Some("pattern_value".to_string()));
        assert_eq!(string_node.format, Some(StringFormat::Date));
        assert_eq!(string_node.min_length, Some(1));
        assert_eq!(string_node.max_length, Some(10));
        assert_eq!(string_node.x_faker, Some("test".to_string()));
    }
}
