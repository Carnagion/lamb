use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::hash::Hash;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ident<T>(T, Option<usize>);

impl<T> Ident<T> {
    pub fn free(var: T) -> Self {
        Self(var, None)
    }

    pub fn var(&self) -> &T {
        &self.0
    }

    pub fn id(&self) -> Option<usize> {
        self.1
    }

    pub fn rebind(&mut self, id: Option<usize>) {
        self.1 = id;
    }
}

impl<T: Clone + Eq + Hash> Ident<T> {
    pub fn bound(var: T, ids: &mut HashMap<T, usize>) -> (Self, usize) {
        let mut ident = Self::free(var.clone());
        let next_id = ids.entry(var)
            .or_default();
        let current_id = *next_id;
        ident.1 = Some(current_id);
        *next_id += 1;
        (ident, current_id)
    }
}

impl<T: Display> Display for Ident<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.var())
    }
}