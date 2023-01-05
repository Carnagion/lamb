use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term<'s> {
    Var(&'s str),
    Abs(&'s str, Box<Self>),
    App(Box<Self>, Box<Self>),
}

impl<'s> Term<'s> {
    pub fn var(name: &'s str) -> Self {
        Self::Var(name)
    }

    pub fn abs(param: &'s str, body: Self) -> Self {
        Self::Abs(param, Box::new(body))
    }

    pub fn app(func: Self, arg: Self) -> Self {
        Self::App(Box::new(func), Box::new(arg))
    }
}

impl Display for Term<'_> {
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