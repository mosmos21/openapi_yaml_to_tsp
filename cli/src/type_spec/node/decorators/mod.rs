mod built_in;
mod http;
mod method_decorator_node;
mod openapi;
mod route_decorator_node;

pub use built_in::*;
pub use http::*;
pub use method_decorator_node::*;
pub use openapi::*;
pub use route_decorator_node::*;
use std::fmt::{Debug, Display};

pub trait LibInfo {
    fn get_lib_name(&self) -> Option<&'static str>;
    fn get_namespace(&self) -> Option<&'static str>;
}

pub trait TypeSpecDecorator: Debug + Display + LibInfo {}
