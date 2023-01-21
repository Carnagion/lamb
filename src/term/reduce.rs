//! Traits and functions for β-reduction of [Term]s.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;
use std::iter;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::Term;

pub mod normal;
pub use normal::*;

/// Represents a β-reduction strategy for [Term]s.
/// 
/// The only associated function required when `impl`ementing this trait is [BetaReduce::beta_reduce_step].
/// The other associated functions have default implementations that rely on [BetaReduce::beta_reduce_step], but can be overridden with custom implementations if necessary.
pub trait BetaReduce<T> {
    /// Performs one step of β-reduction on the [Term] in-place, and returns a value indicating whether reduction was performed or not.
    /// 
    /// Implementations of this function should return `false` if the [Term] is in β-normal form (i.e. no more β-reduction is possible).
    fn beta_reduce_step(&self, term: &mut Term<T>) -> bool;

    /// Attempts to fully β-reduce the [Term] in-place until it reaches β-normal form, and returns the number of reduction steps performed.
    fn beta_reduce(&self, term: &mut Term<T>) -> usize {
        iter::from_fn(|| self.beta_reduce_step(term).then_some(())).count()
    }

    /// Attempts to β-reduce the [Term] in-place until it reaches β-normal form or the predicate returns `false`, and returns the number of reduction steps performed.
    fn beta_reduce_while<P>(&self, term: &mut Term<T>, mut predicate: P) -> usize
    where
        P: FnMut(&Term<T>, usize) -> bool, {
            (0..).into_iter()
                .take_while(|count| predicate(term, *count) && self.beta_reduce_step(term))
                .count()
        }
    
    /// Attempts to β-reduce the [Term] in-place until it reaches β-normal form or the number of reduction steps performed crosses a limit, and returns the latter.
    fn beta_reduce_limit(&self, term: &mut Term<T>, limit: usize) -> usize {
        self.beta_reduce_while(term, |_, count| count < limit)
    }
}

/// A wrapper around a variable indicating whether it is free or bound.
/// 
/// Free variables are represented using their original identifier.
/// Bound variables are represented using their De Bruijn index, starting from 0.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Var<T> {
    /// A bound variable represented as a De Bruijn index.
    Bound(usize),
    /// A free variable represented as its original identifier.
    Free(T),
}

/// Represents possible errors that can occur when converting a [LocalNamelessTerm] to a regular (classic) [Term].
/// 
/// In most cases, there is no possibility of these errors occurring, as all functions that produce or modify [LocalNamelessTerm]s do so in a controlled, deterministic fashion.
/// The only way these errors could arise is if [LocalNamelessTerm]s were constructed or modified manually (and incorrectly).
#[derive(Debug)]
pub enum LocalNamelessError {
    /// The De Bruijn index of a variable is out-of-bounds.
    /// 
    /// This implementation represents free variables using [Var::Free].
    /// Therefore, this error can only occur if the De Bruijn index of a variable is greater than or equal to the number of abstractions it resides in, making it bound to a non-existent formal parameter.
    InvalidVarIndex(usize),
    /// The formal parameter of an abstraction is a [Var::Bound] and not a [Var::Free].
    /// 
    /// This implementation stores all formal parameters of abstractions as a [Var::Free] to retain identifier information.
    /// Therefore, this error can only occur if a formal parameter is incorrectly stored as a [Var::Bound], making it impossible to retrieve its original identifier.
    InvalidAbsParam(usize),
}

/// The locally nameless representation of a [Term].
/// 
/// Variables are wrapped in [Var]s, which avoids the need for α-conversion when substituting or β-reducing [Term]s.
/// However, fully β-reducing a [Term] using this implementation requires two extra steps - converting the [Term] to a [LocalNamelessTerm] and back.
pub type LocalNamelessTerm<T> = Term<Var<T>>;

impl<T: Clone> LocalNamelessTerm<T> {
    /// Fully β-reduces the [LocalNamelessTerm] in-place using the specified [BetaReduce] `impl`ementation.
    pub fn beta_reduce<B: BetaReduce<Var<T>>>(&mut self, reducer: &B) -> usize {
        reducer.beta_reduce(self)
    }

    /// β-reduces the [LocalNamelessTerm] in-place using the specified [BetaReduce] `impl`ementation while a predicate holds true.
    pub fn beta_reduce_while<B, P>(&mut self, predicate: P, reducer: &B) -> usize
    where
        B: BetaReduce<Var<T>>,
        P: FnMut(&Self, usize) -> bool, {
            reducer.beta_reduce_while(self, predicate)
        }
    
    /// β-reduces the [LocalNamelessTerm] in-place up to a certain limit using the specified [BetaReduce] `impl`ementation.
    pub fn beta_reduce_limit<B: BetaReduce<Var<T>>>(&mut self, limit: usize, reducer: &B) -> usize {
        reducer.beta_reduce_limit(self, limit)
    }

    /// β-reduces the [LocalNamelessTerm] once using the specified [BetaReduce] `impl`ementation.
    pub fn beta_reduce_step<B: BetaReduce<Var<T>>>(&mut self, reducer: &B) -> bool {
        reducer.beta_reduce_step(self)
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

impl<T: Clone + Eq + Hash> LocalNamelessTerm<T> {
    /// Replaces the [LocalNamelessTerm]'s free variables in-place with the specified bindings.
    /// 
    /// Free variables that are not part of the provided bindings are left untouched.
    pub fn rebind<'t>(&'t mut self, binds: &mut HashMap<T, Self>) {
        match self {
            Self::Var(Var::Bound(_)) => (),
            Self::Var(Var::Free(var)) => if let Some(term) = binds.get(var) {
                *self = term.clone();
            },
            Self::Abs(_, body) => body.rebind(binds),
            Self::App(func, arg) => {
                func.rebind(binds);
                arg.rebind(binds);
            },
        }
    }
}

impl<T: Clone + Eq> From<&Term<T>> for LocalNamelessTerm<T> {
    fn from(classic: &Term<T>) -> Self {
        classic.to_local_nameless(&mut VecDeque::new())
    }
}

/// A wrapper around a β-reduced [Term], storing along with it the number of reduction steps performed.
#[derive(Debug)]
pub struct ReducedTerm<T> {
    /// The number of β-reduction steps performed when β-reducing the [Term].
    pub count: usize,
    /// The β-reduced [Term].
    pub term: Term<T>,
}

impl<T> AsRef<Term<T>> for ReducedTerm<T> {
    fn as_ref(&self) -> &Term<T> {
        &self.term
    }
}

impl<T> Deref for ReducedTerm<T> {
    type Target = Term<T>;

    fn deref(&self) -> &Self::Target {
        &self.term
    }
}

impl<T> DerefMut for ReducedTerm<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.term
    }
}

impl<T: Clone + Eq> Term<T> {
    /// Returns a fully β-reduced version of the [Term] wrapped in a [ReducedTerm] using the specified [BetaReduce] `impl`ementation.
    pub fn beta_reduced<B: BetaReduce<Var<T>>>(&self, reducer: &B) -> ReducedTerm<T> {
        let mut local_nameless = LocalNamelessTerm::from(self);
        ReducedTerm {
            count: local_nameless.beta_reduce(reducer),
            term: (&local_nameless).try_into().unwrap(),
        }
    }

    /// Returns a version of the [Term] β-reduced using the specified [BetaReduce] `impl`ementation until the predicate returns `false`, wrapped in a [ReducedTerm].
    pub fn beta_reduced_while<B, P>(&self, predicate: P, reducer: &B) -> ReducedTerm<T>
    where
        B: BetaReduce<Var<T>>,
        P: FnMut(&LocalNamelessTerm<T>, usize) -> bool, {
            let mut local_nameless = LocalNamelessTerm::from(self);
            ReducedTerm {
                count: local_nameless.beta_reduce_while::<B, P>(predicate, reducer),
                term: (&local_nameless).try_into().unwrap(),
            }
        }

    /// Returns a version of the [Term] β-reduced up to a certain limit using the specified [BetaReduce] `impl`ementation, wrapped in a [ReducedTerm].
    pub fn beta_reduced_limit<B: BetaReduce<Var<T>>>(&self, limit: usize, reducer: &B) -> ReducedTerm<T> {
        let mut local_nameless = LocalNamelessTerm::from(self);
        ReducedTerm {
            count: local_nameless.beta_reduce_limit::<B>(limit, reducer),
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