use std::fmt::Display;

use crate::type_spec::node::{IdentifierNode, ModelContentNode};

#[derive(Debug)]
pub struct ModelAliasNode {
    pub identifier: IdentifierNode,
    pub alias_type: ModelContentNode,
}

impl Display for ModelAliasNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "alias {} = {};", self.identifier, self.alias_type)
    }
}
