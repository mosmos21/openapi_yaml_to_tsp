use crate::openapi_parser::node as openapi_node;
use crate::type_spec::node as type_spec_node;
use crate::type_spec::node::IdentifierNode;

fn build_array_node(array: &openapi_node::ArrayNode) -> type_spec_node::ModelContentNode {
    type_spec_node::ModelContentNode::Array(type_spec_node::ArrayModelNode {
        item_type: Box::new(build_model_content_node(&array.items)),
    })
}

fn build_union_node(one_of: &openapi_node::OneOfNode) -> type_spec_node::ModelContentNode {
    type_spec_node::ModelContentNode::Union(
        one_of
            .items
            .iter()
            .map(|item| build_model_content_node(item))
            .collect(),
    )
}

fn build_intersection_node(all_of: &openapi_node::AllOfNode) -> type_spec_node::ModelContentNode {
    type_spec_node::ModelContentNode::Intersect(
        all_of
            .items
            .iter()
            .map(|item| build_model_content_node(item))
            .collect(),
    )
}

fn build_model_ref_node(ref_node: &openapi_node::RefNode) -> type_spec_node::ModelContentNode {
    type_spec_node::ModelContentNode::ModelRef(IdentifierNode::from("ModelRefName"))
}

fn build_array_property_node(
    key: &String,
    value: &openapi_node::ArrayNode,
    required: bool,
) -> type_spec_node::RecordPropertyNode {
    type_spec_node::RecordPropertyNode {
        decorators: Box::new(vec![]),
        key: type_spec_node::RecordPropertyKey::Identifier(IdentifierNode::from(key)),
        value: build_array_node(&value),
        required,
    }
}

fn build_object_property_node(
    key: &String,
    value: &openapi_node::ObjectNode,
    required: bool,
) -> type_spec_node::RecordPropertyNode {
    type_spec_node::RecordPropertyNode {
        decorators: Box::new(vec![]),
        key: type_spec_node::RecordPropertyKey::Identifier(IdentifierNode::from(key)),
        value: build_record_model_node(&value),
        required,
    }
}

fn build_union_property_node(
    key: &String,
    value: &openapi_node::OneOfNode,
    required: bool,
) -> type_spec_node::RecordPropertyNode {
    type_spec_node::RecordPropertyNode {
        decorators: Box::new(vec![]),
        key: type_spec_node::RecordPropertyKey::Identifier(IdentifierNode::from(key)),
        value: build_union_node(value),
        required,
    }
}

fn build_string_literal_enum_property_node(
    key: &String,
    values: &Vec<String>,
    required: bool,
) -> type_spec_node::RecordPropertyNode {
    let value = type_spec_node::ModelContentNode::Union(
        values
            .iter()
            .map(|value| type_spec_node::ModelContentNode::StringLiteral(value.clone()))
            .collect(),
    );
    type_spec_node::RecordPropertyNode {
        decorators: Box::new(vec![]),
        key: type_spec_node::RecordPropertyKey::Identifier(IdentifierNode::from(key)),
        value,
        required,
    }
}

fn build_string_property_node(
    key: &String,
    value: &openapi_node::StringNode,
    required: bool,
) -> type_spec_node::RecordPropertyNode {
    if let Some(values) = &value.string_enum {
        return build_string_literal_enum_property_node(key, values, required);
    }

    let mut decorators: Vec<Box<dyn type_spec_node::RecordPropertyDecorator>> = vec![];

    if let Some(pattern) = &value.pattern {
        decorators.push(Box::new(type_spec_node::decorators::PatternDecorator {
            value: pattern.clone(),
        }));
    }
    if let Some(format) = &value.format {
        decorators.push(Box::new(type_spec_node::decorators::FormatDecorator {
            value: format.to_string(),
        }));
    }
    if let Some(min_length) = &value.min_length {
        decorators.push(Box::new(type_spec_node::decorators::MinLengthDecorator {
            value: min_length.clone(),
        }));
    }
    if let Some(max_length) = &value.max_length {
        decorators.push(Box::new(type_spec_node::decorators::MaxLengthDecorator {
            value: max_length.clone(),
        }));
    }

    type_spec_node::RecordPropertyNode {
        decorators: Box::new(decorators),
        key: type_spec_node::RecordPropertyKey::Identifier(IdentifierNode::from(key)),
        value: build_type_node(type_spec_node::TypeNode::String),
        required,
    }
}

fn build_integer_property_node(
    key: &String,
    value: &openapi_node::IntegerNode,
    required: bool,
) -> type_spec_node::RecordPropertyNode {
    type_spec_node::RecordPropertyNode {
        decorators: Box::new(vec![]),
        key: type_spec_node::RecordPropertyKey::Identifier(IdentifierNode::from(key)),
        value: build_type_node(type_spec_node::TypeNode::Int32),
        required,
    }
}

fn build_number_property_node(
    key: &String,
    value: &openapi_node::NumberNode,
    required: bool,
) -> type_spec_node::RecordPropertyNode {
    type_spec_node::RecordPropertyNode {
        decorators: Box::new(vec![]),
        key: type_spec_node::RecordPropertyKey::Identifier(IdentifierNode::from(key)),
        value: build_number_type_node(value),
        required,
    }
}

fn build_boolean_property_node(
    key: &String,
    value: &openapi_node::BooleanNode,
    required: bool,
) -> type_spec_node::RecordPropertyNode {
    type_spec_node::RecordPropertyNode {
        decorators: Box::new(vec![]),
        key: type_spec_node::RecordPropertyKey::Identifier(IdentifierNode::from(key)),
        value: build_type_node(type_spec_node::TypeNode::Boolean),
        required,
    }
}

fn build_intersection_property_node(
    key: &String,
    value: &openapi_node::AllOfNode,
    required: bool,
) -> type_spec_node::RecordPropertyNode {
    type_spec_node::RecordPropertyNode {
        decorators: Box::new(vec![]),
        key: type_spec_node::RecordPropertyKey::Identifier(IdentifierNode::from(key)),
        value: build_intersection_node(value),
        required,
    }
}

fn build_model_ref_property_node(
    key: &String,
    value: &openapi_node::RefNode,
    required: bool,
) -> type_spec_node::RecordPropertyNode {
    type_spec_node::RecordPropertyNode {
        decorators: Box::new(vec![]),
        key: type_spec_node::RecordPropertyKey::Identifier(IdentifierNode::from(key)),
        value: build_model_ref_node(value),
        required,
    }
}

fn build_record_model_node(obj: &openapi_node::ObjectNode) -> type_spec_node::ModelContentNode {
    let properties = obj
        .properties
        .iter()
        .map(|property| match &property.value {
            openapi_node::DataModelNode::Array(array) => {
                build_array_property_node(&property.key, array, property.required)
            }
            openapi_node::DataModelNode::Object(obj) => {
                build_object_property_node(&property.key, obj, property.required)
            }
            openapi_node::DataModelNode::OneOf(items) => {
                build_union_property_node(&property.key, items, property.required)
            }
            openapi_node::DataModelNode::String(str) => {
                build_string_property_node(&property.key, str, property.required)
            }
            openapi_node::DataModelNode::Integer(int) => {
                build_integer_property_node(&property.key, int, property.required)
            }
            openapi_node::DataModelNode::Number(num) => {
                build_number_property_node(&property.key, num, property.required)
            }
            openapi_node::DataModelNode::Boolean(bool) => {
                build_boolean_property_node(&property.key, bool, property.required)
            }
            openapi_node::DataModelNode::AllOf(items) => {
                build_intersection_property_node(&property.key, items, property.required)
            }
            openapi_node::DataModelNode::Ref(ref_node) => {
                build_model_ref_property_node(&property.key, ref_node, property.required)
            }
        })
        .collect();

    type_spec_node::ModelContentNode::Record(type_spec_node::RecordModelNode {
        properties: Box::new(properties),
    })
}

fn build_type_node(t: type_spec_node::TypeNode) -> type_spec_node::ModelContentNode {
    type_spec_node::ModelContentNode::Type(t)
}

fn build_number_type_node(t: &openapi_node::NumberNode) -> type_spec_node::ModelContentNode {
    if let Some(format) = &t.format {
        match format {
            openapi_node::NumberFormat::Float => {
                type_spec_node::ModelContentNode::Type(type_spec_node::TypeNode::Float32)
            }
            openapi_node::NumberFormat::Double => {
                type_spec_node::ModelContentNode::Type(type_spec_node::TypeNode::Float64)
            }
        }
    } else {
        type_spec_node::ModelContentNode::Type(type_spec_node::TypeNode::Int32)
    }
}

fn build_model_content_node(
    data_mode_node: &openapi_node::DataModelNode,
) -> type_spec_node::ModelContentNode {
    match data_mode_node {
        openapi_node::DataModelNode::Array(array) => build_array_node(array),
        openapi_node::DataModelNode::Object(obj) => build_record_model_node(obj),
        openapi_node::DataModelNode::OneOf(one_of) => build_union_node(one_of),
        openapi_node::DataModelNode::String(_) => build_type_node(type_spec_node::TypeNode::String),
        openapi_node::DataModelNode::Integer(_) => build_type_node(type_spec_node::TypeNode::Int32),
        openapi_node::DataModelNode::Number(num) => build_number_type_node(num),
        openapi_node::DataModelNode::Boolean(_) => {
            build_type_node(type_spec_node::TypeNode::Boolean)
        }
        openapi_node::DataModelNode::AllOf(all_of) => build_intersection_node(all_of),
        openapi_node::DataModelNode::Ref(ref_node) => build_model_ref_node(ref_node),
    }
}

pub fn build_model_node(object_node: &openapi_node::ObjectNode) -> type_spec_node::ModelNode {
    let name = object_node
        .title
        .clone()
        .unwrap_or("UnknownModel".to_string());

    if let type_spec_node::ModelContentNode::Record(record) = build_record_model_node(object_node) {
        type_spec_node::ModelNode { name, record }
    } else {
        panic!("Invalid model node");
    }
}
