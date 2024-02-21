use std::path::PathBuf;

use crate::OpenAPINode;

#[derive(Debug)]
pub struct OpenAPIFileNode {
    pub path: PathBuf,
    pub contents: Box<Vec<OpenAPINode>>,
}
