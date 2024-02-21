use std::path::PathBuf;

use super::model_node::{build_import_lib_nodes_from_model_content_node, build_model_content_node};
use crate::compiler::CompilerEnv;
use crate::openapi_parser::node as openapi_node;
use crate::type_spec::node::{self as type_spec_node, IdentifierNode};

pub fn build_model_alias_node(
    data_mode_node: &openapi_node::DataModelNode,
) -> type_spec_node::ModelAliasNode {
    let title = data_mode_node
        .title()
        .unwrap_or("UnknownModelAlias".to_string());
    let alias_type = build_model_content_node(data_mode_node);

    type_spec_node::ModelAliasNode {
        identifier: IdentifierNode::from(title),
        alias_type,
    }
}

pub fn build_import_lib_nodes_model_alias_node(
    node: &type_spec_node::ModelAliasNode,
    current_file_path: &PathBuf,
    env: &CompilerEnv,
) -> Vec<type_spec_node::ImportLibNode> {
    build_import_lib_nodes_from_model_content_node(&node.alias_type, current_file_path, env)
}
