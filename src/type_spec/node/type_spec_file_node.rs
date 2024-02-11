use crate::type_spec::node::*;
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TypeSpecFileNode {
    pub path: PathBuf,
    pub imports: Box<Vec<ImportLibNode>>,
    pub namespaces: Box<Vec<UsingNamespaceNode>>,
    pub contents: Box<Vec<TypeSpecNode>>,
}

impl TypeSpecFileNode {
    pub fn new(
        path: PathBuf,
        imports: Vec<ImportLibNode>,
        namespaces: Vec<UsingNamespaceNode>,
        contents: Vec<TypeSpecNode>,
    ) -> Self {
        TypeSpecFileNode {
            path,
            imports: Box::new(imports),
            namespaces: Box::new(namespaces),
            contents: Box::new(contents),
        }
    }
}

impl Display for TypeSpecFileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let imports = self
            .imports
            .iter()
            .map(|i| format!("{}", i))
            .collect::<Vec<String>>()
            .join("\n");
        let namespaces = self
            .namespaces
            .iter()
            .map(|u| format!("{}", u))
            .collect::<Vec<String>>()
            .join("\n");
        let contents = self
            .contents
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<String>>()
            .join("\n\n");

        write!(f, "{}\n\n{}\n\n{}\n", imports, namespaces, contents)
    }
}
