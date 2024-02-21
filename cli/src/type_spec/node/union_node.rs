use std::fmt::Debug;

#[derive(Debug)]
pub struct UnionNode<T: Debug> {
    items: Box<Vec<T>>,
}
