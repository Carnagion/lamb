use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug, Eq, PartialEq)]
pub enum Var<T> {
    Bound(usize),
    Free(T),
}

impl<T: Display> Display for Var<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Var::Bound(index) => write!(formatter, "{}", index),
            Var::Free(name) => write!(formatter, "{}?", name),
        }
    }
}