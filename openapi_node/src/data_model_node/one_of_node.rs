use crate::common::{YamlHash, YamlWithKey};
use crate::DataModelNode;

#[derive(Debug, Clone)]
pub struct OneOfNode {
    pub title: Option<String>,
    pub items: Vec<DataModelNode>,
}

impl<'a> TryFrom<YamlWithKey<'a>> for OneOfNode {
    type Error = String;

    fn try_from((yaml, key): YamlWithKey<'a>) -> Result<Self, Self::Error> {
        let hash = YamlHash::new(
            yaml.as_hash()
                .ok_or("[OneOfNode::try_from] Expected hash")?,
        );
        let title = key.cloned().or(hash.get_string("title"));
        let items = hash
            .get_vec("oneOf")
            .ok_or("[OneOfNode::try_from] oneOf is required")?
            .iter()
            .map(|item| DataModelNode::try_from((item, key)))
            .collect::<Result<Vec<DataModelNode>, String>>()?;

        Ok(Self { title, items })
    }
}
