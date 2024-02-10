use crate::type_spec::node::*;
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TypeSpecFileNode {
    pub path: PathBuf,
    pub imports: Box<Vec<String>>,
    pub uses: Box<Vec<String>>,
    pub contents: Box<Vec<TypeSpecNode>>,
}

impl TypeSpecFileNode {
    pub fn new(path: PathBuf, contents: Vec<TypeSpecNode>) -> Self {
        TypeSpecFileNode {
            path,
            imports: Box::new(vec!["@typespec/http".to_string()]),
            uses: Box::new(vec!["TypeSpec.Http".to_string()]),
            contents: Box::new(contents),
        }
    }
}

impl Display for TypeSpecFileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let imports = self
            .imports
            .iter()
            .map(|i| format!("import \"{}\";", i))
            .collect::<Vec<String>>()
            .join("\n");
        let uses = self
            .uses
            .iter()
            .map(|u| format!("using {};", u))
            .collect::<Vec<String>>()
            .join("\n");
        let contents = self
            .contents
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<String>>()
            .join("\n\n");

        write!(f, "{}\n\n{}\n\n{}\n", imports, uses, contents)
    }
}
