#[derive(Debug, Eq, PartialEq)]
pub enum Var<T> {
    Bound(usize),
    Free(T),
}