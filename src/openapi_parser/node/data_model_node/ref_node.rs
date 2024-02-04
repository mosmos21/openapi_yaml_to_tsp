use crate::openapi_parser::node::data_model_node::DataModelNode;
use yaml_rust::{yaml, Yaml};

#[derive(Debug)]
pub enum RefNode {
    ComponentRef(ComponentRefNode),
    FileRef(FileRefNode),
}

#[derive(Debug)]
pub struct ComponentRefNode {
    #[allow(dead_code)]
    component_name: String,
}

#[derive(Debug)]
pub struct FileRefNode {
    #[allow(dead_code)]
    file_path: String,
}

pub fn build_ref_node(hash: &yaml::Hash) -> Option<DataModelNode> {
    if let Some(ref_path) = hash
        .get(&Yaml::String("$ref".to_string()))
        .and_then(|v| v.as_str())
    {
        let node = if ref_path.starts_with("#/components/schemas/") {
            let component_name = ref_path.replace("#/components/schemas/", "").to_string();

            RefNode::ComponentRef(ComponentRefNode { component_name })
        } else {
            RefNode::FileRef(FileRefNode {
                file_path: ref_path.to_string(),
            })
        };

        Some(DataModelNode::Ref(node))
    } else {
        None
    }
}
