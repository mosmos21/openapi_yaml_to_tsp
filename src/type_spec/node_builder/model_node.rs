use crate::openapi_parser::node as openapi_node;
use crate::type_spec::node as type_spec_node;
use crate::type_spec::node::RecordPropertyKey;

fn build_array_node(array: &openapi_node::ArrayNode) -> type_spec_node::ModelContentNode {
    todo!()
}

fn build_object_node(obj: &openapi_node::ObjectNode) -> type_spec_node::ModelContentNode {
    todo!()
}

fn build_model_content_node(
    data_mode_node: &openapi_node::DataModelNode,
) -> type_spec_node::ModelContentNode {
    match data_mode_node {
        openapi_node::DataModelNode::Object(obj) => build_object_node(obj),
        openapi_node::DataModelNode::String(_) => {
            type_spec_node::ModelContentNode::Type(type_spec_node::TypeNode::String)
        }
        openapi_node::DataModelNode::Integer(_) => {
            type_spec_node::ModelContentNode::Type(type_spec_node::TypeNode::Int32)
        }
        openapi_node::DataModelNode::Number(_) => {
            type_spec_node::ModelContentNode::Type(type_spec_node::TypeNode::Float32)
        }
        openapi_node::DataModelNode::Boolean(_) => {
            type_spec_node::ModelContentNode::Type(type_spec_node::TypeNode::Boolean)
        }
        openapi_node::DataModelNode::AllOf(_) => unimplemented!(),
        _ => unimplemented!(),
    }
}

pub fn build_model_node(object_node: &openapi_node::ObjectNode) -> type_spec_node::ModelNode {
    type_spec_node::ModelNode {
        name: object_node
            .title
            .clone()
            .unwrap_or("UnknownModel".to_string()),
        record: type_spec_node::RecordModelNode {
            properties: Box::new(vec![]),
        },
    }
}
