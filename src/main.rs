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

    let root_path = args[1].as_str();
    dbg!(&root_path);
    let root_dir = fs::canonicalize(root_path).unwrap();
    dbg!(&root_dir);

    let files = yaml_loader::load_yaml(&root_dir).flatten_files();
    let result = files
        .into_iter()
        .map(|file| openapi_parser::parse_yaml_file(file))
        .collect::<Vec<_>>();

    write_log("openapi_node.log", &result);
}
