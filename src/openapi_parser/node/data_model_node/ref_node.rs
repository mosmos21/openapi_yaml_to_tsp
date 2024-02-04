use crate::openapi_parser::node::data_model_node::DataModelNode;
use yaml_rust::{yaml, Yaml};

#[derive(Debug)]
pub struct RefNode {
    #[allow(dead_code)]
    ref_path: String,
}

pub fn build_ref_node(hash: &yaml::Hash) -> Option<DataModelNode> {
    let ref_key = &Yaml::String("$ref".to_string());
    if let Some(ref_path) = hash.get(ref_key).and_then(|v| v.as_str()) {
        Some(DataModelNode::Ref(RefNode {
            ref_path: ref_path.to_string(),
        }))
    } else {
        None
    }
}
