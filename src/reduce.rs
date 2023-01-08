use std::collections::VecDeque;
use std::mem;

use crate::term::Term;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Var<T> {
    Bound(usize),
    Free(T),
}

type LocalNamelessTerm<T> = Term<Var<T>>;

impl<T: Clone> LocalNamelessTerm<T> {
    fn reduce(&mut self) {
        match self {
            Self::Var(_) => (),
            Self::Abs(_, body) => body.reduce(),
            Self::App(func, arg) => match func.as_mut() {
                Self::Abs(_, body) => {
                    body.reduce();
                    body.open(0, arg);
                    *self = mem::replace(body, Self::Var(Var::Bound(0)));
                },
                func => {
                    func.reduce();
                    arg.reduce();
                },
            },
        }
    }

    fn open(&mut self, depth: usize, replacement: &Self) {
        match self {
            Self::Var(Var::Bound(index)) => if *index == depth {
                *self = replacement.clone();
            } else if *index > depth {
                *index -= 1;
            },
            Self::Var(Var::Free(_)) => (),
            Self::Abs(_, body) => body.open(depth + 1, replacement),
            Self::App(func, arg) => {
                func.open(depth, replacement);
                arg.open(depth, replacement);
            }
        }
    }

    fn into_classic(self, vars: &mut VecDeque<T>) -> Term<T> {
        match self {
            Self::Var(Var::Bound(index)) => Term::var(vars[index].clone()),
            Self::Var(Var::Free(var)) => Term::var(var),
            Self::Abs(param, body) => match param {
                Var::Bound(_) => unreachable!(),
                Var::Free(param) => {
                    vars.push_front(param.clone());
                    let term = Term::abs(param, body.into_classic(vars));
                    vars.pop_front();
                    term
                },
            },
            Self::App(func, arg) => Term::app(func.into_classic(vars), arg.into_classic(vars)),
        }
    }
}

impl<T: Clone + Eq> Term<T> {
    fn into_local_nameless(self, vars: &mut VecDeque<T>) -> LocalNamelessTerm<T> {
        match self {
            Self::Var(var) => match vars.iter().position(|param| param == &var) {
                Some(index) => LocalNamelessTerm::var(Var::Bound(index)),
                None => LocalNamelessTerm::var(Var::Free(var)),
            },
            Self::Abs(param, body) => {
                vars.push_front(param.clone());
                let term = LocalNamelessTerm::abs(Var::Free(param), body.into_local_nameless(vars));
                vars.pop_front();
                term
            },
            Self::App(func, arg) => LocalNamelessTerm::app(func.into_local_nameless(vars), arg.into_local_nameless(vars)),
        }
    }
}