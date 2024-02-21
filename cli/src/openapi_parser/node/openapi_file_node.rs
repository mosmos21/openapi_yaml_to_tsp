use std::path::PathBuf;

use yaml_rust::yaml;

use crate::openapi_parser::node::*;

#[derive(Debug)]
pub struct OpenAPIFileNode {
    pub path: PathBuf,
    pub contents: Box<Vec<OpenAPINode>>,
}

#[derive(Debug)]
pub enum OpenAPINode {
    Metadata(MetadataNode),
    Info(InfoNode),
    Servers(Box<Vec<ServerNode>>),
    Tags(Box<Vec<TagNode>>),
    Paths(Box<Vec<PathNode>>),
    Operation(OperationNode),
    DataModel(DataModelNode),
    Parameters(Box<Vec<ParameterNode>>),
    Example(ExampleNode),
    Unknown(Box<yaml::Hash>),
}
