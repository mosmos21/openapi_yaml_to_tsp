use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub enum YamlNode {
    YamlDirectory(YamlDirectory),
    YamlFile(YamlFile),
}

#[derive(Debug)]
pub struct YamlDirectory {
    pub path: PathBuf,
    pub children: Box<Vec<YamlNode>>,
}

#[derive(Debug, Clone)]
pub struct YamlFile {
    pub path: PathBuf,
    pub content: Box<Vec<Yaml>>,
}

fn build_nest_str(nest: usize) -> String {
    vec!["  "; nest].join("")
}

impl YamlNode {
    fn to_str(&self, nest: usize) -> String {
        match self {
            YamlNode::YamlDirectory(d) => d.to_str(nest),
            YamlNode::YamlFile(y) => y.to_str(nest),
        }
    }
    pub fn flatten_files(self) -> Vec<YamlFile> {
        match self {
            YamlNode::YamlDirectory(d) => {
                let mut files = vec![];
                for child in d.children.into_iter() {
                    files.append(&mut child.flatten_files());
                }
                files
            }
            YamlNode::YamlFile(y) => vec![y],
        }
    }
}

impl YamlDirectory {
    fn to_str(&self, nest: usize) -> String {
        let nest_str = build_nest_str(nest);

        let children = self
            .children
            .iter()
            .map(|c| c.to_str(nest + 1))
            .collect::<Vec<_>>();

        format!(
            "{}{}\n{}",
            nest_str,
            self.path.to_str().expect("failed to convert path to str"),
            children.join("\n")
        )
    }
}

impl YamlFile {
    fn to_str(&self, nest: usize) -> String {
        let nest_str = build_nest_str(nest);

        format!(
            "{}{}",
            nest_str,
            self.path.to_str().expect("failed to convert path to str")
        )
    }
}

impl Display for YamlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str(0))
    }
}

impl Display for YamlDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str(0))
    }
}

impl Display for YamlFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str(0))
    }
}

fn load_file(path: &PathBuf) -> Vec<Yaml> {
    let content = fs::read_to_string(path).expect("failed to read file");

    YamlLoader::load_from_str(&content).expect("failed to parse yaml")
}

pub fn load_yaml(root: &PathBuf) -> YamlNode {
    if root.is_file() {
        return YamlNode::YamlFile(YamlFile {
            path: root.clone(),
            content: Box::new(load_file(root)),
        });
    }

    let entries = fs::read_dir(root).expect("read_dir call failed");

    let paths = entries
        .into_iter()
        .map(|entry| entry.expect("failed to read_entry").path())
        .filter(|path| {
            path.is_dir() || path.to_str().map(|s| s.ends_with(".yaml")).unwrap_or(false)
        })
        .collect::<Vec<_>>();

    let nodes = paths
        .into_iter()
        .map(|path| load_yaml(&path))
        .collect::<Vec<_>>();

    YamlNode::YamlDirectory(YamlDirectory {
        path: root.clone(),
        children: Box::new(nodes),
    })
}
