use crate::compiler::CompilerEnv;
use crate::openapi_parser::node as openapi_node;
use crate::type_spec::node as type_spec_node;
use crate::type_spec::node_builder::build_namespace_node;
use crate::type_spec::node_builder::model_node::build_model_node;

type BuildContentResult = (
    Option<type_spec_node::TypeSpecNode>,
    Vec<openapi_node::OpenAPINode>,
);

fn build_content_namespace_node(
    mut contents: Vec<openapi_node::OpenAPINode>,
    env: &CompilerEnv,
) -> BuildContentResult {
    if let Some(openapi_node::OpenAPINode::Info(info_node)) = contents.get(0) {
        let namespace_node = build_namespace_node(info_node, env);
        contents.remove(0);
        (
            Some(type_spec_node::TypeSpecNode::NameSpace(namespace_node)),
            contents,
        )
    } else {
        (None, contents)
    }
}

fn build_content_model_node(
    mut contents: Vec<openapi_node::OpenAPINode>,
    _env: &CompilerEnv,
) -> BuildContentResult {
    if let Some(openapi_node::OpenAPINode::DataModel(openapi_node::DataModelNode::Object(ojb))) =
        contents.get(0)
    {
        let model_node = build_model_node(ojb);
        contents.remove(0);
        (
            Some(type_spec_node::TypeSpecNode::Model(model_node)),
            contents,
        )
    } else {
        (None, contents)
    }
}

fn build_content_unknown_node(
    mut contents: Vec<openapi_node::OpenAPINode>,
    _env: &CompilerEnv,
) -> BuildContentResult {
    contents.remove(0);
    (None, contents)
}

fn build_content(
    mut contents: Vec<openapi_node::OpenAPINode>,
    env: &CompilerEnv,
) -> BuildContentResult {
    vec![
        build_content_namespace_node,
        build_content_model_node,
        build_content_unknown_node,
    ]
    .iter()
    .fold((None, contents), |(node, contents), builder| {
        if node.is_some() {
            (node, contents)
        } else {
            builder(contents, env)
        }
    })
}

fn build_contents(
    mut contents: Vec<openapi_node::OpenAPINode>,
    env: &CompilerEnv,
) -> Vec<type_spec_node::TypeSpecNode> {
    let mut result = Vec::new();

    while contents.len() > 0 {
        let len = contents.len();
        let (node, new_contents) = build_content(contents, env);
        if let Some(node) = node {
            result.push(node);
        }
        if new_contents.len() == len {
            panic!("invalid contents");
        }
        contents = new_contents;
    }

    result
}

pub fn build_type_spec_file_node(
    openapi_file_node: openapi_node::OpenAPIFileNode,
    env: &CompilerEnv,
) -> type_spec_node::TypeSpecFileNode {
    let openapi_node::OpenAPIFileNode { path, contents } = openapi_file_node;

    let path_str = path
        .to_str()
        .map(|s| s.replace(".yaml", ".tsp"))
        .expect("invalid path");
    let contents = build_contents(contents.into_iter().collect(), env);

    type_spec_node::TypeSpecFileNode::new(path_str.into(), contents)
}
