use crate::common::{YamlHash, YamlWithKey};
use crate::DataModelNode;

#[derive(Debug, Clone)]
pub struct ArrayNode {
    pub title: Option<String>,
    pub items: Box<DataModelNode>,
}

impl<'a> TryFrom<YamlWithKey<'a>> for ArrayNode {
    type Error = String;

    fn try_from((yaml, key): YamlWithKey<'a>) -> Result<Self, Self::Error> {
        let hash = YamlHash::new(
            yaml.as_hash()
                .ok_or("[ArrayNode::try_from] Expected hash")?,
        );
        let items = hash
            .get_value("items")
            .ok_or("[ArrayNode::try_from] Expected items")?;

        let title = key.cloned().or(hash.get_string("title"));
        let items = DataModelNode::try_from((items, key))?;

        Ok(Self {
            title,
            items: Box::new(items),
        })
    }
}
