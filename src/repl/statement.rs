//! [Statement]s that can be evaluated at run-time.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use crate::Term;

/// Represents a statement that can be evaluated (such as by an interpreter or compiler).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement<T> {
    /// A binding of a [Term] to an identifier.
    Bind(T, Term<T>),
}

impl<T: Display> Display for Statement<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Bind(name, term) => write!(formatter, "{} = {};", name, term),
        }
    }
}