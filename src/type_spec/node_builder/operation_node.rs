use crate::openapi_parser::{node as openapi_node, ParameterPosition};
use crate::type_spec::node as type_spec_node;
use crate::type_spec::node::OperationDecorator;
use crate::type_spec::node_builder::model_node::{build_model_content_node, build_model_node};
use log::warn;
use std::process;

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

impl From<&openapi_node::ParameterNode> for type_spec_node::ParameterNode {
    fn from(parameter: &openapi_node::ParameterNode) -> Self {
        let mut decorators: Vec<Box<dyn type_spec_node::ParameterDecorator>> = vec![];
        match parameter.position {
            ParameterPosition::Path => {
                decorators.push(Box::new(type_spec_node::decorators::PathDecorator {}));
            }
            ParameterPosition::Header => {
                decorators.push(Box::new(type_spec_node::decorators::HeaderDecorator {}));
            }
            ParameterPosition::Cookie => {
                warn!("cookie parameter is not supported")
            }
            _ => {}
        }

        let name = parameter.name.clone();
        let type_model = build_model_content_node(&parameter.schema);

        type_spec_node::ParameterNode {
            decorators: Box::new(decorators),
            name,
            type_model,
        }
    }
}

fn build_response_node(response: &openapi_node::ResponseNode) -> type_spec_node::ModelContentNode {
    let mut properties = vec![];

    properties.push(type_spec_node::RecordPropertyNode {
        decorators: Box::new(vec![Box::new(
            type_spec_node::decorators::StatusCodeDecorator {},
        )]),
        key: type_spec_node::RecordPropertyKey::Identifier(type_spec_node::IdentifierNode::from(
            "statusCode",
        )),
        value: type_spec_node::ModelContentNode::IntegerLiteral(response.status.get_code().into()),
        required: true,
    });

    if let Some(content_type) = &response.content_type {
        properties.push(type_spec_node::RecordPropertyNode {
            decorators: Box::new(vec![Box::new(
                type_spec_node::decorators::HeaderDecorator {},
            )]),
            key: type_spec_node::RecordPropertyKey::Identifier(
                type_spec_node::IdentifierNode::from("contentType"),
            ),
            value: type_spec_node::ModelContentNode::StringLiteral(content_type.to_string()),
            required: true,
        });
    }

    if let Some(body) = &response.schema {
        properties.push(type_spec_node::RecordPropertyNode {
            decorators: Box::new(vec![Box::new(type_spec_node::decorators::BodyDecorator {})]),
            key: type_spec_node::RecordPropertyKey::Identifier(
                type_spec_node::IdentifierNode::from("body"),
            ),
            value: build_model_content_node(body),
            required: true,
        });
    }

    type_spec_node::ModelContentNode::Record(type_spec_node::RecordModelNode {
        properties: Box::new(properties),
    })
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
    let parameters = operation_node
        .parameters
        .iter()
        .map(|p| type_spec_node::ParameterNode::from(p))
        .collect::<Vec<_>>();

    let responses = operation_node
        .responses
        .iter()
        .map(|res| build_response_node(res))
        .collect::<Vec<_>>();

    type_spec_node::OperationNode {
        name: get_operation_name(&operation_node.op).to_string(),
        decorators: Box::new(decorators),
        parameters: Box::new(parameters),
        responses: Box::new(responses),
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
