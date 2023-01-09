use std::collections::VecDeque;
use std::iter;
use std::mem;

use crate::term::Term;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Var<T> {
    Bound(usize),
    Free(T),
}

pub enum LocalNamelessError {
    InvalidVarIndex(usize),
    InvalidAbsParam(usize),
}

pub type LocalNamelessTerm<T> = Term<Var<T>>;

impl<T: Clone> LocalNamelessTerm<T> {
    pub fn reduce(&mut self) -> usize {
        iter::from_fn(|| self.reduce_step().then_some(())).count()
    }

    pub fn reduce_until<P>(&mut self, mut predicate: P) -> usize
    where
        P: FnMut(&Self, usize) -> bool, {
            (1..).into_iter()
                .take_while(|count| predicate(self, *count) && self.reduce_step())
                .last()
                .unwrap_or_default()
    }

    pub fn reduce_limit(&mut self, limit: usize) -> usize {
        self.reduce_until(|_, count| count >= limit)
    }

    pub fn reduce_step(&mut self) -> bool {
        match self {
            Self::Var(_) => false,
            Self::Abs(_, body) => body.reduce_step(),
            Self::App(func, arg) => match func.as_mut() {
                Self::Abs(_, body) => {
                    let body_reduced = body.reduce_step();
                    let body_opened = body.open(0, arg);
                    *self = mem::replace(body, Self::Var(Var::Bound(0)));
                    body_reduced || body_opened
                },
                func => {
                    let func_reduced = func.reduce_step();
                    let arg_reduced = arg.reduce_step();
                    func_reduced || arg_reduced
                },
            },
        }
    }

    fn open(&mut self, depth: usize, replacement: &Self) -> bool {
        match self {
            Self::Var(Var::Bound(index)) => if *index == depth {
                *self = match replacement {
                    Self::Var(Var::Bound(index)) => Self::Var(Var::Bound(index + depth)),
                    _ => replacement.clone(),
                };
                true
            } else {
                if *index > depth {
                    *index -= 1;
                }
                false
            },
            Self::Var(Var::Free(_)) => false,
            Self::Abs(_, body) => body.open(depth + 1, replacement),
            Self::App(func, arg) => {
                let func_opened = func.open(depth, replacement);
                let arg_opened = arg.open(depth, replacement);
                func_opened || arg_opened
            },
        }
    }

    fn into_classic<'t>(&'t self, vars: &mut VecDeque<&'t T>) -> Result<Term<T>, LocalNamelessError> {
        match self {
            Self::Var(Var::Bound(index)) => match vars.get(*index) {
                Some(&var) => Ok(Term::var(var.clone())),
                None => Err(LocalNamelessError::InvalidVarIndex(*index)),
            },
            Self::Var(Var::Free(var)) => Ok(Term::var(var.clone())),
            Self::Abs(param, body) => match param {
                Var::Bound(index) => Err(LocalNamelessError::InvalidAbsParam(*index)),
                Var::Free(param) => {
                    vars.push_front(param);
                    let term = Term::abs(param.clone(), body.into_classic(vars)?);
                    vars.pop_front();
                    Ok(term)
                },
            },
            Self::App(func, arg) => Ok(Term::app(func.into_classic(vars)?, arg.into_classic(vars)?)),
        }
    }
}

impl<T: Clone + Eq> From<&Term<T>> for LocalNamelessTerm<T> {
    fn from(classic: &Term<T>) -> Self {
        classic.into_local_nameless(&mut VecDeque::new())
    }
}

impl<T: Clone + Eq> Term<T> {
    fn into_local_nameless<'t>(&'t self, vars: &mut VecDeque<&'t T>) -> LocalNamelessTerm<T> {
        match self {
            Self::Var(var) => match vars.iter().position(|&param| param == var) {
                Some(index) => LocalNamelessTerm::var(Var::Bound(index)),
                None => LocalNamelessTerm::var(Var::Free(var.clone())),
            },
            Self::Abs(param, body) => {
                vars.push_front(param);
                let term = LocalNamelessTerm::abs(Var::Free(param.clone()), body.into_local_nameless(vars));
                vars.pop_front();
                term
            },
            Self::App(func, arg) => LocalNamelessTerm::app(func.into_local_nameless(vars), arg.into_local_nameless(vars)),
        }
    }
}

impl<T: Clone> TryFrom<&LocalNamelessTerm<T>> for Term<T> {
    type Error = LocalNamelessError;

    fn try_from(local_nameless: &LocalNamelessTerm<T>) -> Result<Self, Self::Error> {
        local_nameless.into_classic(&mut VecDeque::new())
    }
}