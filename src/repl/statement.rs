use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use crate::Term;

#[derive(Debug, Eq, PartialEq)]
pub enum Statement<T> {
    Bind(T, Term<T>),
}

impl<T> Statement<T> {
    pub fn bind(name: T, term: Term<T>) -> Self {
        Self::Bind(name, term)
    }
}

impl<T: Display> Display for Statement<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Bind(name, term) => write!(formatter, "{} = {};", name, term),
        }
    }
}