use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

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
                (abs @ Self::Abs(_, _), app @ Self::App(_, _)) => write!(formatter, "({}) ({})", abs, app),
                (abs @ Self::Abs(_, _), _) => write!(formatter, "({}) {}", abs, arg),
                (_, app @ Self::App(_, _)) => write!(formatter, "{} ({})", func, app),
                _ => write!(formatter, "{} {}", func, arg),
            },
        }
    }
}