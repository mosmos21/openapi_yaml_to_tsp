use yaml_rust::yaml;

use crate::*;

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
