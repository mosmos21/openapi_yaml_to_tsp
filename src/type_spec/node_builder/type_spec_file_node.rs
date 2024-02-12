use crate::compiler::CompilerEnv;
use crate::openapi_parser::node as openapi_node;
use crate::type_spec::node as type_spec_node;
use crate::type_spec::node_builder::build_contents;
use crate::type_spec::node_builder::type_spec_node::{
    build_import_lib_nodes_from_type_spec_node, build_using_namespace_nodes_from_type_spec_node,
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

fn build_import_and_name_spaces(
    contents: &Vec<type_spec_node::TypeSpecNode>,
    current_file_path: &PathBuf,
    env: &CompilerEnv,
) -> (
    Vec<type_spec_node::ImportLibNode>,
    Vec<type_spec_node::UsingNamespaceNode>,
) {
    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();

    contents.iter().for_each(|node| {
        s1.extend(build_import_lib_nodes_from_type_spec_node(
            &node,
            current_file_path,
            env,
        ));
        s2.extend(build_using_namespace_nodes_from_type_spec_node(&node, env));
    });

    (
        s1.iter().cloned().collect::<Vec<_>>(),
        s2.iter().cloned().collect::<Vec<_>>(),
    )
}

pub fn build_type_spec_file_node(
    openapi_file_node: openapi_node::OpenAPIFileNode,
    env: &CompilerEnv,
) -> type_spec_node::TypeSpecFileNode {
    let openapi_node::OpenAPIFileNode { path, contents } = openapi_file_node;

    let path_str = &path
        .to_str()
        .map(|s| s.replace(".yaml", ".tsp"))
        .expect("invalid path");
    let file_name = Path::new(path_str)
        .file_name()
        .expect("invalid file name")
        .to_str()
        .expect("invalid file name");
    let contents = build_contents(contents.into_iter().collect(), file_name, env);
    let (imports, namespaces) = build_import_and_name_spaces(&contents, &path, env);

    type_spec_node::TypeSpecFileNode::new(path_str.into(), imports, namespaces, contents)
}
