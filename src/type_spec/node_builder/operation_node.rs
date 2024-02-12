use crate::openapi_parser::node as openapi_node;
use crate::type_spec::node as type_spec_node;
use crate::type_spec::node::OperationDecorator;

impl From<&openapi_node::Operation> for type_spec_node::decorators::MethodDecoratorNode {
    fn from(operation: &openapi_node::Operation) -> Self {
        match operation {
            openapi_node::Operation::Get => type_spec_node::decorators::MethodDecoratorNode::new(
                type_spec_node::decorators::Method::Get,
            ),
            openapi_node::Operation::Post => type_spec_node::decorators::MethodDecoratorNode::new(
                type_spec_node::decorators::Method::Post,
            ),
            openapi_node::Operation::Put => type_spec_node::decorators::MethodDecoratorNode::new(
                type_spec_node::decorators::Method::Put,
            ),
            openapi_node::Operation::Delete => {
                type_spec_node::decorators::MethodDecoratorNode::new(
                    type_spec_node::decorators::Method::Delete,
                )
            }
            openapi_node::Operation::Patch => type_spec_node::decorators::MethodDecoratorNode::new(
                type_spec_node::decorators::Method::Patch,
            ),
        }
    }
}

fn get_operation_name<'a>(operation: &openapi_node::Operation) -> &'a str {
    match operation {
        openapi_node::Operation::Get => "list",
        openapi_node::Operation::Post => "create",
        openapi_node::Operation::Patch => "update",
        openapi_node::Operation::Put => "update",
        openapi_node::Operation::Delete => "delete",
    }
}

pub fn build_operation_node(
    operation_node: &openapi_node::OperationNode,
) -> type_spec_node::OperationNode {
    let decorators: Vec<Box<dyn OperationDecorator>> = vec![Box::new(
        type_spec_node::decorators::MethodDecoratorNode::from(&operation_node.op),
    )];
    let parameters = vec![];

    type_spec_node::OperationNode {
        name: get_operation_name(&operation_node.op).to_string(),
        decorators: Box::new(decorators),
        parameters: Box::new(parameters),
    }
}

pub fn build_import_lib_nodes_from_operation_node(
    operation_node: &type_spec_node::OperationNode,
) -> Vec<type_spec_node::ImportLibNode> {
    let mut imports = vec![];

    imports.extend(
        operation_node
            .decorators
            .iter()
            .filter_map(|node| node.get_lib_name())
            .map(type_spec_node::ImportLibNode::from)
            .collect::<Vec<_>>(),
    );

    imports
}
