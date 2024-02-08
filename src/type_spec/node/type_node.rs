use std::fmt::Display;

#[derive(Debug)]
pub enum TypeNode {
    Boolean,
    Int32,
    Int64,
    Float32,
    Float64,
    String,
}

impl Display for TypeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().to_lowercase())
    }
}
