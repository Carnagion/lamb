use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use crate::term::Term;

#[derive(Debug, Eq, PartialEq)]
pub enum Statement<'s> {
    Bind(&'s str, Term<'s>),
}

impl Display for Statement<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Bind(name, term) => write!(formatter, "{} = {}.", name, term),
        }
    }
}