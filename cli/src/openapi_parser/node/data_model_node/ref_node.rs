use yaml_rust::{yaml, Yaml};

use crate::openapi_parser::node::data_model_node::DataModelNode;

#[derive(Debug, Clone)]
pub enum RefNode {
    ComponentRef(ComponentRefNode),
    FileRef(FileRefNode),
}

#[derive(Debug, Clone)]
pub struct ComponentRefNode {
    pub component_name: String,
}

#[derive(Debug, Clone)]
pub struct FileRefNode {
    pub file_path: String,
}

pub fn build_ref_node(hash: &yaml::Hash) -> Option<DataModelNode> {
    if let Some(ref_path) = hash
        .get(&Yaml::String("$ref".to_string()))
        .and_then(|v| v.as_str())
    {
        let node = if ref_path.ends_with(".yaml") {
            RefNode::FileRef(FileRefNode {
                file_path: ref_path.to_string(),
            })
        } else if ref_path.starts_with("#/components/schemas/") {
            let component_name = ref_path.replace("#/components/schemas/", "").to_string();

            RefNode::ComponentRef(ComponentRefNode { component_name })
        } else {
            RefNode::ComponentRef(ComponentRefNode {
                component_name: ref_path.to_string(),
            })
        };

        Some(DataModelNode::Ref(node))
    } else {
        None
    }
}
