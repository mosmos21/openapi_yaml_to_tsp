mod compiler;
mod openapi_parser;
mod type_spec;
mod yaml_loader;

use std::fs;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let root_dir = fs::canonicalize(args[1].clone()).unwrap();
    dbg!(&root_dir);

    compiler::compile(&root_dir);
}
