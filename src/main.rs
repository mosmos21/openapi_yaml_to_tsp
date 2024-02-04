mod compiler;
mod openapi_parser;
mod yaml_loader;

use std::fmt::Debug;
use std::fs::{self, File};
use std::io::Write;

fn write_log<T: Debug>(log_file_name: &str, content: &T) {
    fs::remove_file(log_file_name).expect("Failed to remove file");

    let mut log_file = File::create(log_file_name).unwrap();

    log_file
        .write_all(format!("{:#?}", content).as_bytes())
        .unwrap();
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let root_dir = fs::canonicalize(args[1].clone()).unwrap();
    dbg!(&root_dir);

    let nodes = compiler::compile(&root_dir);

    write_log("openapi_node.log", &nodes);
}
