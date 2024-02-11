use std::fmt::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UsingNamespaceNode {
    pub namespace: String,
}

impl UsingNamespaceNode {
    pub fn new(namespace: String) -> Self {
        UsingNamespaceNode { namespace }
    }
}

impl Display for UsingNamespaceNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "using {};", self.namespace)
    }
}
