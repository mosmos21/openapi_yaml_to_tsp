use crate::openapi_parser::node::info_node::InfoNode;

#[derive(Debug)]
pub struct ServiceNode {
    #[allow(dead_code)]
    pub title: Option<String>,
    #[allow(dead_code)]
    pub version: Option<String>,
}

impl From<&InfoNode> for ServiceNode {
    fn from(info: &InfoNode) -> Self {
        ServiceNode {
            title: Some(info.title.clone()),
            version: Some(info.version.clone()),
        }
    }
}
