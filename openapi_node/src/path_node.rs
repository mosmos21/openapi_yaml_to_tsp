use yaml_rust::Yaml;

use crate::common::{YamlEntry, YamlHash};

#[derive(Debug)]
pub struct PathNode {
    pub path: String,
    pub ref_file_path: String,
}

impl<'a> TryFrom<YamlEntry<'a>> for PathNode {
    type Error = String;

    fn try_from((key, value): YamlEntry<'a>) -> Result<Self, Self::Error> {
        let path = key
            .as_str()
            .ok_or("[PathNode::try_from] path must be a string")?
            .to_string();
        let ref_file_path = value
            .as_hash()
            .ok_or("[PathNode::try_from] ref_file_path must be a hash")
            .map(YamlHash::new)
            .and_then(|hash| {
                hash.get_string("$ref")
                    .ok_or("[PathNode::try_from] $ref is required")
            })?;

        Ok(PathNode {
            path,
            ref_file_path,
        })
    }
}

pub struct PathsNode {
    pub paths: Box<Vec<PathNode>>,
}

impl PathsNode {
    pub fn new(paths: Vec<PathNode>) -> Self {
        PathsNode {
            paths: Box::new(paths),
        }
    }
}

impl TryFrom<&Yaml> for PathsNode {
    type Error = String;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        let hash = yaml
            .as_hash()
            .ok_or("[PathsNode::try_from] paths must be a hash")
            .map(YamlHash::new)?;
        let paths = hash
            .get_value("paths")
            .ok_or("[PathsNode::try_from] paths is required")?;

        paths
            .as_hash()
            .ok_or("[PathsNode::try_from] paths must be a hash".to_string())
            .and_then(|hash| {
                hash.iter()
                    .map(PathNode::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
            .map(|paths| PathsNode::new(paths))
    }
}
