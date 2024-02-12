use crate::openapi_parser::node as openapi_node;
use crate::type_spec::node as type_spec_node;

pub fn build_enum_node(
    string_node: &openapi_node::StringNode,
    current_file_name: &str,
) -> type_spec_node::EnumNode {
    let title = string_node
        .title
        .to_owned()
        .unwrap_or(current_file_name.to_string());
    let items = string_node.string_enum.to_owned().unwrap_or(vec![]);

    type_spec_node::EnumNode {
        title,
        items: Box::new(items),
    }
}
