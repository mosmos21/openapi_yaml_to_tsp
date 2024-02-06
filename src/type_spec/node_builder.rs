use crate::compiler::CompilerEnv;
use crate::openapi_parser::OpenAPIFileNode;
use crate::type_spec::node::TypeSpecFileNode;
use std::path::PathBuf;

pub fn build_from_openapi_file_node(
    file_node: &OpenAPIFileNode,
    _env: &CompilerEnv,
) -> TypeSpecFileNode {
    let file_path = file_node
        .path
        .to_str()
        .expect("invalid file path")
        .replace(".yaml", ".tsp");

    TypeSpecFileNode {
        path: PathBuf::from(file_path),
        contents: Box::new(vec![]),
    }
}
