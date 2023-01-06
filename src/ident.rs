use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ident<T>(T, Option<usize>);

impl<T> Ident<T> {
    pub fn var(&self) -> &T {
        &self.0
    }

    pub fn index(&self) -> Option<usize> {
        self.1
    }

    pub fn free(ident: T) -> Self {
        Self(ident, None)
    }
}

impl<T: Display> Display for Ident<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.var())
    }
}