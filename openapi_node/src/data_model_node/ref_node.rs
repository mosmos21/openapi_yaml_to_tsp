use crate::common::{YamlHash, YamlWithKey};

#[derive(Debug, Clone, PartialEq)]
pub enum RefNode {
    ComponentRef(ComponentRefNode),
    FileRef(FileRefNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentRefNode {
    pub component_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileRefNode {
    pub file_path: String,
}

impl<'a> TryFrom<YamlWithKey<'a>> for RefNode {
    type Error = String;

    fn try_from((yaml, _): YamlWithKey<'a>) -> Result<Self, Self::Error> {
        let hash = YamlHash::new(yaml.as_hash().ok_or("[RefNode::try_from] Expected hash")?);
        let ref_path = hash
            .get_string("$ref")
            .ok_or("[RefNode::try_from] $ref is required")?;

        if ref_path.ends_with(".yaml") {
            Ok(Self::FileRef(FileRefNode {
                file_path: ref_path.to_string(),
            }))
        } else {
            let component_name = ref_path.replace("#/components/schemas/", "").to_string();

            Ok(Self::ComponentRef(ComponentRefNode { component_name }))
        }
    }
}

#[cfg(test)]
mod test {
    use yaml_rust::YamlLoader;

    use super::*;

    #[test]
    fn test_component_ref_node_try_from_yaml1() {
        let yaml = "$ref: \"#/components/schemas/Tag\"";

        let yaml = YamlLoader::load_from_str(yaml).unwrap();
        let yaml = &yaml[0];

        let key = None;
        let result = RefNode::try_from((yaml, key));
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            RefNode::ComponentRef(ComponentRefNode {
                component_name: "Tag".to_string(),
            })
        );
    }

    #[test]
    fn test_component_ref_node_try_from_yaml2() {
        let yaml = "$ref: \"Tag\"";

        let yaml = YamlLoader::load_from_str(yaml).unwrap();
        let yaml = &yaml[0];

        let key = None;
        let result = RefNode::try_from((yaml, key));
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            RefNode::ComponentRef(ComponentRefNode {
                component_name: "Tag".to_string(),
            })
        );
    }

    #[test]
    fn test_file_ref_node_try_from_yaml() {
        let yaml = "$ref: \"./tag.yaml\"";

        let yaml = YamlLoader::load_from_str(yaml).unwrap();
        let yaml = &yaml[0];

        let key = None;
        let result = RefNode::try_from((yaml, key));
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            RefNode::FileRef(FileRefNode {
                file_path: "./tag.yaml".to_string(),
            })
        );
    }
}
