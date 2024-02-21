use std::fmt::Display;

use crate::type_spec::node::common::string_literal;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ImportLibNode {
    pub lib_name: String,
}

#[derive(Debug)]
pub struct ImportLibNodes {
    pub items: Box<Vec<ImportLibNode>>,
}

impl Display for ImportLibNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.items
                .iter()
                .map(|node| format!("{}", node))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl ImportLibNode {
    pub fn new(lib_name: String) -> Self {
        ImportLibNode { lib_name }
    }
}

impl From<String> for ImportLibNode {
    fn from(lib_name: String) -> Self {
        ImportLibNode {
            lib_name: lib_name.to_string(),
        }
    }
}

impl From<&str> for ImportLibNode {
    fn from(lib_name: &str) -> Self {
        ImportLibNode {
            lib_name: lib_name.to_string(),
        }
    }
}

impl Display for ImportLibNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "import {};", string_literal(&self.lib_name))
    }
}
