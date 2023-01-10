use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::term::Term;

/// A wrapper around a variable indicating whether it is [free](https://en.wikipedia.org/wiki/Lambda_calculus#Free_and_bound_variables) or [bound](https://en.wikipedia.org/wiki/Lambda_calculus#Free_and_bound_variables).
/// 
/// Free variables are represented using their original identifier.
/// Bound variables are represented using their [De Bruijn index](https://en.wikipedia.org/wiki/De_Bruijn_index), starting from 0.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Var<T> {
    /// A bound variable represented as a De Bruijn index.
    Bound(usize),
    /// A free variable represented as its original identifier.
    Free(T),
}

#[derive(Debug)]
pub enum LocalNamelessError {
    InvalidVarIndex(usize),
    InvalidAbsParam(usize),
}

/// The [locally nameless representation](https://www.chargueraud.org/softs/ln/) of a [Term].
/// 
/// Variables are wrapped in [Var]s, which avoids the need for α-conversion when substituting or β-reducing [Term]s.
pub type LocalNamelessTerm<T> = Term<Var<T>>;

impl<T: Clone> LocalNamelessTerm<T> {
    pub fn reduce(&mut self) -> usize {
        iter::from_fn(|| self.reduce_step().then_some(())).count()
    }

    pub fn reduce_while<P>(&mut self, mut predicate: P) -> usize
    where
        P: FnMut(&Self, usize) -> bool, {
            (0..).into_iter()
                .take_while(|count| predicate(self, *count) || self.reduce_step())
                .count()
        }
    
    pub fn reduce_limit(&mut self, limit: usize) -> usize {
        self.reduce_while(|_, count| count < limit)
    }

    pub fn reduce_step(&mut self) -> bool {
        match self {
            Self::Var(_) => false,
            Self::Abs(_, body) => body.reduce_step(),
            Self::App(func, arg) => match func.as_mut() {
                Self::Abs(_, body) => {
                    body.reduce_step();
                    body.open(0, arg);
                    *self = mem::replace(body, Self::Var(Var::Bound(0)));
                    true
                },
                func => {
                    let func_reduced = func.reduce_step();
                    let arg_reduced = arg.reduce_step();
                    func_reduced || arg_reduced
                },
            },
        }
    }

    fn open(&mut self, depth: usize, replacement: &Self) {
        match self {
            Self::Var(Var::Bound(index)) => match (*index).cmp(&depth) {
                Ordering::Equal => *self = replacement.shifted(0, depth),
                Ordering::Greater => *index -= 1,
                Ordering::Less => (),
            },
            Self::Var(Var::Free(_)) => (),
            Self::Abs(_, body) => body.open(depth + 1, replacement),
            Self::App(func, arg) => {
                func.open(depth, replacement);
                arg.open(depth, replacement);
            },
        }
    }

    fn shifted(&self, depth: usize, amount: usize) -> Self {
        match self {
            Self::Var(Var::Bound(index)) => if *index >= depth {
                Self::var(Var::Bound(*index + amount))
            } else {
                Self::var(Var::Bound(*index))
            },
            Self::Var(Var::Free(var)) => Self::var(Var::Free(var.clone())),
            Self::Abs(param, body) => Self::abs(param.clone(), body.shifted(depth + 1, amount)),
            Self::App(func, arg) => Self::app(func.shifted(depth, amount), arg.shifted(depth, amount)),
        }
    }

    fn to_classic<'t>(&'t self, vars: &mut VecDeque<&'t T>) -> Result<Term<T>, LocalNamelessError> {
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
                    let term = Term::abs(param.clone(), body.to_classic(vars)?);
                    vars.pop_front();
                    Ok(term)
                },
            },
            Self::App(func, arg) => Ok(Term::app(func.to_classic(vars)?, arg.to_classic(vars)?)),
        }
    }
}

impl<T: Clone + Eq> From<&Term<T>> for LocalNamelessTerm<T> {
    fn from(classic: &Term<T>) -> Self {
        classic.to_local_nameless(&mut VecDeque::new())
    }
}

#[derive(Debug)]
pub struct ReducedTerm<T> {
    count: usize,
    term: Term<T>,
}

impl<T> ReducedTerm<T> {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn term(&self) -> &Term<T> {
        &self.term
    }
}

impl<T> AsRef<Term<T>> for ReducedTerm<T> {
    fn as_ref(&self) -> &Term<T> {
        self.term()
    }
}

impl<T> Deref for ReducedTerm<T> {
    type Target = Term<T>;

    fn deref(&self) -> &Self::Target {
        self.term()
    }
}

impl<T> DerefMut for ReducedTerm<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.term
    }
}

impl<T: Clone + Eq> Term<T> {
    pub fn reduced(&self) -> ReducedTerm<T> {
        let mut local_nameless = LocalNamelessTerm::from(self);
        ReducedTerm {
            count: local_nameless.reduce(),
            term: (&local_nameless).try_into().unwrap(),
        }
    }

    pub fn reduced_until<P>(&self, predicate: P) -> ReducedTerm<T>
    where
        P: FnMut(&LocalNamelessTerm<T>, usize) -> bool, {
            let mut local_nameless = LocalNamelessTerm::from(self);
            ReducedTerm {
                count: local_nameless.reduce_while(predicate),
                term: (&local_nameless).try_into().unwrap(),
            }
        }

    pub fn reduced_limit(&self, limit: usize) -> ReducedTerm<T> {
        let mut local_nameless = LocalNamelessTerm::from(self);
        ReducedTerm {
            count: local_nameless.reduce_limit(limit),
            term: (&local_nameless).try_into().unwrap(),
        }
    }

    fn to_local_nameless<'t>(&'t self, vars: &mut VecDeque<&'t T>) -> LocalNamelessTerm<T> {
        match self {
            Self::Var(var) => match vars.iter().position(|&param| param == var) {
                Some(index) => LocalNamelessTerm::var(Var::Bound(index)),
                None => LocalNamelessTerm::var(Var::Free(var.clone())),
            },
            Self::Abs(param, body) => {
                vars.push_front(param);
                let term = LocalNamelessTerm::abs(Var::Free(param.clone()), body.to_local_nameless(vars));
                vars.pop_front();
                term
            },
            Self::App(func, arg) => LocalNamelessTerm::app(func.to_local_nameless(vars), arg.to_local_nameless(vars)),
        }
    }
}

impl<T: Clone> TryFrom<&LocalNamelessTerm<T>> for Term<T> {
    type Error = LocalNamelessError;

    fn try_from(local_nameless: &LocalNamelessTerm<T>) -> Result<Self, Self::Error> {
        local_nameless.to_classic(&mut VecDeque::new())
    }
}

impl<T> From<ReducedTerm<T>> for Term<T> {
    fn from(reduced: ReducedTerm<T>) -> Self {
        reduced.term
    }
}