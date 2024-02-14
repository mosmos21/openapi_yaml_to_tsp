use crate::type_spec::node::decorators::{LibInfo, TypeSpecDecorator};
use crate::type_spec::node::{ParameterDecorator, RecordPropertyDecorator};
use std::fmt::Display;

#[derive(Debug)]
pub struct PathDecorator;

impl Display for PathDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@path")
    }
}

impl LibInfo for PathDecorator {
    fn get_lib_name(&self) -> Option<&'static str> {
        Some("@typespec/http")
    }
    fn get_namespace(&self) -> Option<&'static str> {
        Some("TypeSpec.Http")
    }
}

impl TypeSpecDecorator for PathDecorator {}

impl ParameterDecorator for PathDecorator {}

#[derive(Debug)]
pub struct HeaderDecorator;

impl Display for HeaderDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@header")
    }
}

impl LibInfo for HeaderDecorator {
    fn get_lib_name(&self) -> Option<&'static str> {
        Some("@typespec/http")
    }
    fn get_namespace(&self) -> Option<&'static str> {
        Some("TypeSpec.Http")
    }
}

impl TypeSpecDecorator for HeaderDecorator {}

impl ParameterDecorator for HeaderDecorator {}

impl RecordPropertyDecorator for HeaderDecorator {}

#[derive(Debug)]
pub struct StatusCodeDecorator;

impl Display for StatusCodeDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@statusCode")
    }
}

impl LibInfo for StatusCodeDecorator {
    fn get_lib_name(&self) -> Option<&'static str> {
        Some("@typespec/http")
    }
    fn get_namespace(&self) -> Option<&'static str> {
        Some("TypeSpec.Http")
    }
}

impl TypeSpecDecorator for StatusCodeDecorator {}

impl RecordPropertyDecorator for StatusCodeDecorator {}

#[derive(Debug)]
pub struct BodyDecorator;

impl Display for BodyDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@body")
    }
}

impl LibInfo for BodyDecorator {
    fn get_lib_name(&self) -> Option<&'static str> {
        Some("@typespec/http")
    }
    fn get_namespace(&self) -> Option<&'static str> {
        Some("TypeSpec.Http")
    }
}

impl TypeSpecDecorator for BodyDecorator {}

impl RecordPropertyDecorator for BodyDecorator {}
