use std::collections::VecDeque;

use crate::term::Term;

#[derive(Debug, Eq, PartialEq)]
pub enum Var<T> {
    Bound(usize),
    Free(T),
}

pub type LocalNamelessTerm<T> = Term<Var<T>>;

impl<T: Clone + Eq> Term<T> {
    fn into_local_nameless<'t>(&'t self, depth: usize, vars: &mut VecDeque<&'t T>) -> LocalNamelessTerm<T> {
        match self {
            Self::Var(var) => if let Some(index) = vars.iter().position(|&param| param == var) {
                Term::var(Var::Bound(index))
            } else {
                Term::var(Var::Free(var.clone()))
            },
            Self::Abs(param, body) => {
                vars.push_front(param);
                let term = Term::abs(Var::Bound(0), body.into_local_nameless(depth + 1, vars));
                vars.pop_front();
                term
            },
            Self::App(func, arg) => Term::app(func.into_local_nameless(depth, vars), arg.into_local_nameless(depth, vars)),
        }
    }
}

impl<T: Clone + Eq> From<Term<T>> for LocalNamelessTerm<T> {
    fn from(term: Term<T>) -> Self {
        term.into_local_nameless(0, &mut VecDeque::new())
    }
}