use yaml_rust::Yaml;

use crate::common::YamlWithKey;
use crate::data_model_node::*;

#[derive(Debug, Clone)]
pub enum DataModelNode {
    Array(ArrayNode),
    Object(ObjectNode),
    OneOf(OneOfNode),
    String(StringNode),
    Integer(IntegerNode),
    Number(NumberNode),
    Boolean(BooleanNode),
    AllOf(AllOfNode),
    Ref(RefNode),
}

impl DataModelNode {
    pub fn title(&self) -> Option<&String> {
        match self {
            DataModelNode::Array(node) => node.title.as_ref(),
            DataModelNode::Object(node) => node.title.as_ref(),
            DataModelNode::OneOf(node) => node.title.as_ref(),
            DataModelNode::String(node) => node.title.as_ref(),
            DataModelNode::Integer(_) => None,
            DataModelNode::Number(_) => None,
            DataModelNode::Boolean(_) => None,
            DataModelNode::AllOf(node) => node.title.as_ref(),
            DataModelNode::Ref(_) => None,
        }
    }
}

impl<'a> TryFrom<YamlWithKey<'a>> for DataModelNode {
    type Error = String;

    fn try_from(args: YamlWithKey<'a>) -> Result<Self, Self::Error> {
        ObjectNode::try_from(args)
            .map(DataModelNode::Object)
            .or_else(|_| OneOfNode::try_from(args).map(DataModelNode::OneOf))
            .or_else(|_| AllOfNode::try_from(args).map(DataModelNode::AllOf))
            .or_else(|_| ArrayNode::try_from(args).map(DataModelNode::Array))
            .or_else(|_| StringNode::try_from(args).map(DataModelNode::String))
            .or_else(|_| IntegerNode::try_from(args).map(DataModelNode::Integer))
            .or_else(|_| NumberNode::try_from(args).map(DataModelNode::Number))
            .or_else(|_| BooleanNode::try_from(args).map(DataModelNode::Boolean))
            .or_else(|_| RefNode::try_from(args).map(DataModelNode::Ref))
    }
}

impl TryFrom<&Yaml> for DataModelNode {
    type Error = String;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        Self::try_from((yaml, None))
    }
}
