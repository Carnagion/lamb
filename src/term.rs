use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::hash::Hash;

use crate::ident::Ident;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term<T> {
    Var(Ident<T>),
    Abs(Ident<T>, Box<Self>),
    App(Box<Self>, Box<Self>),
}

impl<T> Term<T> {
    pub fn var(name: T) -> Self {
        Self::Var(Ident::free(name))
    }

    pub fn abs(param: T, body: Self) -> Self {
        Self::Abs(Ident::free(param), Box::new(body))
    }

    pub fn app(func: Self, arg: Self) -> Self {
        Self::App(Box::new(func), Box::new(arg))
    }
}

impl<T: Clone + Eq + Hash> Term<T> {
    fn rebind(&mut self, ids: &mut HashMap<T, usize>, scopes: &mut HashMap<Ident<T>, VecDeque<usize>>) {
        match self {
            Self::Var(ident) => {
                scopes.get(ident)
                    .map(|scope| scope.front()
                        .map(|id| ident.rebind(Some(*id))));
            },
            Self::Abs(param, body) => {
                let (bound, id) = Ident::bound(param.var().clone(), ids);
                scopes.entry(param.clone())
                    .or_default()
                    .push_front(id);
                body.rebind(ids, scopes);
                scopes.entry(param.clone())
                    .or_default()
                    .pop_front();
                *param = bound;
            },
            Self::App(func, arg) => {
                func.rebind(ids, scopes);
                arg.rebind(ids, scopes);
            },
        }
    }
}

impl<T: Display> Display for Term<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Var(var) => write!(formatter, "{}", var),
            Self::Abs(param, body) => write!(formatter, "λ{}. {}", param, body),
            Self::App(func, arg) => match (func.as_ref(), arg.as_ref()) {
                (Self::Abs(_, _), Self::Abs(_, _) | Self::App(_, _)) => write!(formatter, "({}) ({})", func, arg),
                (Self::Abs(_, _), _) => write!(formatter, "({}) {}", func, arg),
                (_, Self::Abs(_, _) | Self::App(_, _)) => write!(formatter, "{} ({})", func, arg),
                _ => write!(formatter, "{} {}", func, arg),
            },
        }
    }
}

#[macro_export]
macro_rules! var {
    ($name: ident) => {
        $crate::term::Term::var(stringify!($name))
    };
}

#[macro_export]
macro_rules! abs {
    ($param: ident. $body: expr) => {
        $crate::term::Term::abs(stringify!($param), $body)
    };
    ($param: ident $($rest: ident)+. $body: expr) => {{
        $crate::term::Term::abs(stringify!($param), abs!($($rest)+. $body))
    }};
}

#[macro_export]
macro_rules! app {
    ($func: expr, $($arg: expr),+) => {{
        let mut app = $func;
        $(app = $crate::term::Term::app(app, $arg);)+
        app
    }};
}

#[macro_export]
macro_rules! lambda {
    (λ $param: ident $($params: ident)+. $($body: tt)+) => {
        $crate::term::Term::abs(stringify!($param), lambda!(λ$($params)+. $($body)+))
    };
    (λ $param: ident. $($body: tt)+) => {
        $crate::term::Term::abs(stringify!($param), lambda!($($body)+))
    };
    ($func: ident $($args: tt)+) => {
        lambda_fold_apply!($($args)+).into_iter()
            .fold($crate::term::Term::var(stringify!($func)), |func, arg| $crate::term::Term::app(func, arg))
    };
    (($($func: tt)+) $($args: tt)+) => {
        lambda_fold_apply!($($args)+).into_iter()
            .fold(lambda!($($func)+), |func, arg| $crate::term::Term::app(func, arg))
    };
    ($var: ident) => {
        $crate::term::Term::var(stringify!($var))
    };
    (($($term: tt)+)) => {
        lambda!($($term)+)
    };
}

macro_rules! lambda_fold_apply {
    ($func: ident $($args: tt)+) => {
        std::iter::once($crate::term::Term::var(stringify!($func))).chain(lambda_fold_apply!($($args)+))
    };
    (($($func: tt)+) $($args: tt)+) => {
        std::iter::once(lambda!($($func)+)).chain(lambda_fold_apply!($($args)+))
    };
    ($($args: tt)+) => {
        std::iter::once(lambda!($($args)+))
    };
}