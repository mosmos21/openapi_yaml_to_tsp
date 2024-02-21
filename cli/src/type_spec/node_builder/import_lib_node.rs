use crate::openapi_parser::node as openapi_node;
use crate::type_spec::node as type_spec_node;

pub fn build_import_lib_nodes(
    paths: &Vec<openapi_node::PathNode>,
) -> type_spec_node::ImportLibNodes {
    let paths = paths
        .iter()
        .map(|path_node| {
            let file_path = path_node
                .ref_file_path
                .to_str()
                .map(|s| s.to_string())
                .unwrap();

            type_spec_node::ImportLibNode {
                lib_name: file_path.replace(".yaml", ".tsp"),
            }
        })
        .collect();

    type_spec_node::ImportLibNodes {
        items: Box::new(paths),
    }
}
