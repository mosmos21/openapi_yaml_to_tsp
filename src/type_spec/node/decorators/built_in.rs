use crate::type_spec::node::common::string_literal;
use crate::type_spec::node::decorators::{LibInfo, TypeSpecDecorator};
use crate::type_spec::node::{NameSpaceDecorator, RecordPropertyDecorator};
use std::fmt::Display;

#[derive(Debug)]
pub struct MinLengthDecorator {
    pub value: usize,
}

impl Display for MinLengthDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@minLength({})", self.value)
    }
}

impl LibInfo for MinLengthDecorator {
    fn get_lib_name(&self) -> Option<&'static str> {
        None
    }
    fn get_namespace(&self) -> Option<&'static str> {
        None
    }
}

impl TypeSpecDecorator for MinLengthDecorator {}

impl RecordPropertyDecorator for MinLengthDecorator {}

#[derive(Debug)]
pub struct MaxLengthDecorator {
    pub value: usize,
}

impl Display for MaxLengthDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@maxLength({})", self.value)
    }
}

impl LibInfo for MaxLengthDecorator {
    fn get_lib_name(&self) -> Option<&'static str> {
        None
    }
    fn get_namespace(&self) -> Option<&'static str> {
        None
    }
}

impl TypeSpecDecorator for MaxLengthDecorator {}

impl RecordPropertyDecorator for MaxLengthDecorator {}

#[derive(Debug)]
pub struct PatternDecorator {
    pub value: String,
}

impl Display for PatternDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@pattern({})", string_literal(&self.value))
    }
}

impl LibInfo for PatternDecorator {
    fn get_lib_name(&self) -> Option<&'static str> {
        None
    }
    fn get_namespace(&self) -> Option<&'static str> {
        None
    }
}

impl TypeSpecDecorator for PatternDecorator {}

impl RecordPropertyDecorator for PatternDecorator {}

#[derive(Debug)]
pub struct MinimumDecorator {
    pub value: f64,
}

impl Display for MinimumDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@minimum({})", self.value)
    }
}

impl LibInfo for MinimumDecorator {
    fn get_lib_name(&self) -> Option<&'static str> {
        None
    }
    fn get_namespace(&self) -> Option<&'static str> {
        None
    }
}

impl TypeSpecDecorator for MinimumDecorator {}

impl RecordPropertyDecorator for MinimumDecorator {}

#[derive(Debug)]
pub struct MaximumDecorator {
    pub value: f64,
}

impl Display for MaximumDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@maximum({})", self.value)
    }
}

impl LibInfo for MaximumDecorator {
    fn get_lib_name(&self) -> Option<&'static str> {
        None
    }
    fn get_namespace(&self) -> Option<&'static str> {
        None
    }
}

impl TypeSpecDecorator for MaximumDecorator {}

impl RecordPropertyDecorator for MaximumDecorator {}

#[derive(Debug)]
pub struct FormatDecorator {
    pub value: String,
}

impl Display for FormatDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@format({})", self.value)
    }
}

impl LibInfo for FormatDecorator {
    fn get_lib_name(&self) -> Option<&'static str> {
        None
    }
    fn get_namespace(&self) -> Option<&'static str> {
        None
    }
}

impl TypeSpecDecorator for FormatDecorator {}

impl RecordPropertyDecorator for FormatDecorator {}

#[derive(Debug)]
pub struct ServiceDecorator {
    pub title: String,
    pub version: String,
}

impl Display for ServiceDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "@service({{ title: {}, version: {} }})",
            string_literal(&self.title),
            string_literal(&self.version)
        )
    }
}

impl LibInfo for ServiceDecorator {
    fn get_lib_name(&self) -> Option<&'static str> {
        None
    }
    fn get_namespace(&self) -> Option<&'static str> {
        None
    }
}

impl TypeSpecDecorator for ServiceDecorator {}

impl NameSpaceDecorator for ServiceDecorator {}
