use std::collections::VecDeque;

use crate::term::Term;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Var<T> {
    Bound(usize),
    Free(T),
}

pub type LocalNamelessTerm<T> = Term<Var<T>>;

impl<T: Clone> LocalNamelessTerm<T> {
    fn open(&mut self, replacement: &Self) -> &mut Self {
        match self.open_with(0, replacement) {
            Term::Abs(_, body) => body,
            opened => opened,
        }
    }

    fn open_with(&mut self, depth: usize, replacement: &Self) -> &mut Self {
        match self {
            Term::Var(Var::Free(_)) => self,
            Term::Var(Var::Bound(index)) => {
                if *index == depth {
                    *self = replacement.clone();
                }
                self
            },
            Term::Abs(_, body) => {
                body.open_with(depth + 1, replacement);
                self
            },
            Term::App(func, arg) => {
                func.open_with(depth, replacement);
                arg.open_with(depth, replacement);
                self
            },
        }
    }
}

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
                let term = Term::abs(Var::Free(param.clone()), body.into_local_nameless(depth + 1, vars));
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