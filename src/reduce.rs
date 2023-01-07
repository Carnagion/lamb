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
            Self::Abs(_, body) => body,
            opened => opened,
        }
    }

    fn open_with(&mut self, depth: usize, replacement: &Self) -> &mut Self {
        match self {
            Self::Var(Var::Free(_)) => self,
            Self::Var(Var::Bound(index)) => {
                if *index == depth {
                    *self = replacement.clone();
                }
                self
            },
            Self::Abs(_, body) => {
                body.open_with(depth + 1, replacement);
                self
            },
            Self::App(func, arg) => {
                func.open_with(depth, replacement);
                arg.open_with(depth, replacement);
                self
            },
        }
    }
}

impl<T: Clone + Eq> Term<T> {
    fn into_local_nameless<'t>(&'t self, vars: &mut VecDeque<&'t T>) -> LocalNamelessTerm<T> {
        match self {
            Self::Var(var) => if let Some(index) = vars.iter().position(|&param| param == var) {
                Term::var(Var::Bound(index))
            } else {
                Term::var(Var::Free(var.clone()))
            },
            Self::Abs(param, body) => {
                vars.push_front(param);
                let term = Term::abs(Var::Free(param.clone()), body.into_local_nameless(vars));
                vars.pop_front();
                term
            },
            Self::App(func, arg) => Term::app(func.into_local_nameless(vars), arg.into_local_nameless(vars)),
        }
    }
}

impl<T: Clone> LocalNamelessTerm<T> {
    fn into_classic<'t>(&'t self, vars: &mut VecDeque<&'t T>) -> Term<T> {
        match self {
            Self::Var(Var::Free(var)) => Term::var(var.clone()),
            Self::Var(Var::Bound(index)) => Term::var(vars[*index].clone()),
            Self::Abs(param, body) => match param {
                Var::Free(param) => {
                    vars.push_front(param);
                    let term = Term::abs(param.clone(), body.into_classic(vars));
                    vars.pop_front();
                    term
                },
                Var::Bound(_) => unreachable!(),
            },
            Self::App(func, arg) => Term::app(func.into_classic(vars), arg.into_classic(vars)),
        }
    }
}

impl<T: Clone + Eq> From<Term<T>> for LocalNamelessTerm<T> {
    fn from(term: Term<T>) -> Self {
        term.into_local_nameless(&mut VecDeque::new())
    }
}

impl<T: Clone> From<LocalNamelessTerm<T>> for Term<T> {
    fn from(term: LocalNamelessTerm<T>) -> Self {
        term.into_classic(&mut VecDeque::new())
    }
}

#[test]
fn test() {
    use crate::prelude::combinators;
    println!("{:?}", combinators::compose().into_local_nameless(&mut VecDeque::new()));
    assert_eq!(combinators::id(), combinators::id().into_local_nameless(&mut VecDeque::new()).into_classic(&mut VecDeque::new()));
}