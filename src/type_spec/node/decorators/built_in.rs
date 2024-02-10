use crate::type_spec::node::RecordPropertyDecorator;
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

impl RecordPropertyDecorator for MaxLengthDecorator {}

#[derive(Debug)]
pub struct PatternDecorator {
    pub value: String,
}

impl Display for PatternDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@pattern({})", self.value)
    }
}

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

impl RecordPropertyDecorator for FormatDecorator {}
