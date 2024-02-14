use std::fmt::Display;

#[derive(Debug)]
pub enum TypeNode {
    Boolean,
    Int32,
    Int64,
    Float32,
    Float64,
    PlainDate,
    UtcDateTime,
    Byte,
    String,
}

impl Display for TypeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeNode::Boolean => write!(f, "boolean"),
            TypeNode::Int32 => write!(f, "int32"),
            TypeNode::Int64 => write!(f, "int64"),
            TypeNode::Float32 => write!(f, "float32"),
            TypeNode::Float64 => write!(f, "float64"),
            TypeNode::PlainDate => write!(f, "plainDate"),
            TypeNode::UtcDateTime => write!(f, "utcDateTime"),
            TypeNode::Byte => write!(f, "byte"),
            TypeNode::String => write!(f, "string"),
        }
    }
}
