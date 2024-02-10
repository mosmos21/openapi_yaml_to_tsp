use std::fmt::Display;

#[derive(Debug)]
pub struct IdentifierNode {
    pub name: String,
}

impl From<String> for IdentifierNode {
    fn from(name: String) -> Self {
        IdentifierNode { name }
    }
}

impl From<&String> for IdentifierNode {
    fn from(name: &String) -> Self {
        IdentifierNode { name: name.clone() }
    }
}

impl<'a> From<&'a str> for IdentifierNode {
    fn from(name: &'a str) -> Self {
        IdentifierNode {
            name: name.to_string(),
        }
    }
}

impl Display for IdentifierNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
