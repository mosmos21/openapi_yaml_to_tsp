use yaml_rust::Yaml;

#[derive(Debug)]
pub struct ExampleNode {
    pub yaml: Yaml,
}

impl TryFrom<&Yaml> for ExampleNode {
    type Error = String;

    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        Ok(ExampleNode { yaml: yaml.clone() })
    }
}
