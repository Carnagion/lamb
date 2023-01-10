use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub mod reduce;

#[cfg(test)]
mod tests;

/// A lambda calculus [term](https://en.wikipedia.org/wiki/Lambda_calculus#Lambda_terms), which is either a variable, an abstraction, or an application.
/// 
/// [Term]s can be constructed in multiple ways:
/// - The [lambda!](crate::lambda) macro
/// - The [var!](crate::var), [abs!](crate::abs), and [app!](crate::app) macros
/// - The [Term::var], [Term::abs], and [Term::app] associated functions
/// - [Term]'s enum variants
/// 
/// The macros offer some syntactic sugar for the construction of [Term]s, and will suffice for the vast majority of cases.
/// For greater control over how [Term]s are constructed, consider using the associated functions and enum variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term<T> {
    /// A variable, which may be either free or bound to an abstraction's formal parameter.
    Var(T),
    /// An abstraction, binding a formal parameter inside its body.
    Abs(T, Box<Self>),
    /// An application of one [Term] to another.
    App(Box<Self>, Box<Self>),
}

impl<T> Term<T> {
    /// Constructs a variable with the provided identifier (`var`).
    pub fn var(var: T) -> Self {
        Self::Var(var)
    }

    /// Constructs an abstraction, binding the formal parameter (`param`) to the abstraction body (`body`).
    pub fn abs(param: T, body: Self) -> Self {
        Self::Abs(param, Box::new(body))
    }

    /// Constructs an application, applying the [Term] on the left (`func`) to that on the right (`arg`).
    pub fn app(func: Self, arg: Self) -> Self {
        Self::App(Box::new(func), Box::new(arg))
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

/// Constructs a variable using an identifier.
/// 
/// While the actual [Term::Var] variant has no constraint on what is and isn't an identifier, this macro only accepts a regular Rust identifier, and stringifies it.
/// 
/// # Examples
/// 
/// ```
/// use lambda::var;
/// use lambda::term::Term;
/// 
/// let var_term = var!(x);
/// assert_eq!(var_term, Term::var("x"));
/// ```
#[macro_export]
macro_rules! var {
    ($name: ident) => {
        $crate::term::Term::var(stringify!($name))
    };
}

/// Constructs an abstraction with one or more formal parameters and a body.
/// 
/// This macro supports syntax sugar for multiple formal parameters.
/// There must be at least one parameter, and each parameter must be delimited by whitespace, ending with a dot (`.`).
/// The abstraction body comes after all parameters have been declared.
/// 
/// Abstraction bodies extend as far as possible to the right, i.e. `λa. b c a` is interpreted as `λa. (b c a)`.
/// 
/// # Examples
/// 
/// ```
/// use lambda::var;
/// use lambda::abs;
/// use lambda::app;
/// use lambda::term::Term;
/// 
/// let term_a = abs!(x. var!(x));
/// assert_eq!(term_a, Term::abs("x", Term::var("x")));
/// 
/// let term_b = abs!(x y z. app!(var!(x), var!(y), var!(z)));
/// assert_eq!(term_b, Term::abs(
///     "x",
///     Term::abs(
///         "y",
///         Term::abs(
///             "z",
///             Term::app(
///                 Term::app(
///                     Term::var("x"),
///                     Term::var("y"),
///                 ),
///                 Term::var("z"),
///             ),
///         ),
///     ),
/// ));
/// ```
#[macro_export]
macro_rules! abs {
    ($param: ident. $body: expr) => {
        $crate::term::Term::abs(stringify!($param), $body)
    };
    ($param: ident $($rest: ident)+. $body: expr) => {{
        $crate::term::Term::abs(stringify!($param), abs!($($rest)+. $body))
    }};
}

/// Constructs an application from multiple [Term]s.
/// 
/// This macro supports syntax sugar for multiple application.
/// There must be at least two [Term]s to apply together, and each [Term] must be delimited by a comma (`,`).
/// Trailing commas are not permitted.
/// 
/// Application is left-associative, i.e. `a b c d` is interpreted as `((a b) c) d`.
/// 
/// # Examples
/// 
/// ```
/// use lambda::var;
/// use lambda::abs;
/// use lambda::app;
/// use lambda::term::Term;
/// 
/// let term_a = app!(var!(x), var!(y));
/// assert_eq!(term_a, Term::app(
///     Term::var("x"),
///     Term::var("y"),
/// ));
/// 
/// let term_b = app!(var!(a), abs!(b. var!(a)), var!(d));
/// assert_eq!(term_b, Term::app(
///     Term::app(
///         Term::var("a"),
///         Term::abs("b", Term::var("a"))),
///     Term::var("d"),
/// ));
/// ```
#[macro_export]
macro_rules! app {
    ($func: expr, $($arg: expr),+) => {{
        let mut app = $func;
        $(app = $crate::term::Term::app(app, $arg);)+
        app
    }};
}

/// Constructs a [Term] using syntax as close as possible to that of standard untyped lambda calculus.
/// 
/// This macro allows constructing arbitrary [Term]s using standard lambda calculus syntax.
/// It supports syntax sugar for multiple formal parameter and multiple application.
/// 
/// Abstraction bodies extend as far as possible to the right, i.e. `λa. b c a` is interpreted as `λa. (b c a)`.
/// Application is left-associative, i.e. `a b c d` is interpreted as `((a b) c) d`.
/// 
/// Some whitespace is necessary after each `λ`, otherwise Rust will process the `λ` as part of an identifier.
/// i.e. `λx. x` will produce invalid syntax, but `λ x. x` will parse correctly.
/// 
/// Whitespace is also necessary between two terms being applied together, unless one or both of the terms are enclosed in parentheses (`()`).
/// i.e. `xy` will be parsed as a variable `xy`, but `x y` or `x(y)` will be parsed as `x` applied to `y`.
/// 
/// # Examples
/// 
/// ```
/// use lambda::lambda;
/// use lambda::term::Term;
/// 
/// let term_a = lambda!(x);
/// assert_eq!(term_a, Term::var("x"));
/// 
/// let term_b = lambda!(λ x. x);
/// assert_eq!(term_b, Term::abs("x", Term::var("x")));
/// 
/// let term_c = lambda!((λ x y. y (x x y)) (λ x y. y (x x y)));
/// assert_eq!(term_c, Term::app(
///     Term::abs(
///         "x",
///         Term::abs(
///             "y",
///             Term::app(
///                 Term::var("y"),
///                 Term::app(
///                     Term::app(Term::var("x"), Term::var("x")),
///                     Term::var("y"),
///                 ),
///             ),
///         ),
///     ),
///     Term::abs(
///         "x",
///         Term::abs(
///             "y",
///             Term::app(
///                 Term::var("y"),
///                 Term::app(
///                     Term::app(Term::var("x"), Term::var("x")),
///                     Term::var("y"),
///                 ),
///             ),
///         ),
///     ),
/// ));
/// ```
#[macro_export]
macro_rules! lambda {
    (λ $param: ident $($params: ident)+. $($body: tt)+) => {
        $crate::term::Term::abs(stringify!($param), lambda!(λ$($params)+. $($body)+))
    };
    (λ $param: ident. $($body: tt)+) => {
        $crate::term::Term::abs(stringify!($param), lambda!($($body)+))
    };
    ($func: ident $($args: tt)+) => {
        lambda!(~internal $($args)+).into_iter()
            .fold($crate::term::Term::var(stringify!($func)), $crate::term::Term::app)
    };
    (($($func: tt)+) $($args: tt)+) => {
        lambda!(~internal $($args)+).into_iter()
            .fold(lambda!($($func)+), $crate::term::Term::app)
    };
    ($var: ident) => {
        $crate::term::Term::var(stringify!($var))
    };
    (($($term: tt)+)) => {
        lambda!($($term)+)
    };
    (~internal $func: ident $($args: tt)+) => {
        std::iter::once($crate::term::Term::var(stringify!($func))).chain(lambda!(~internal $($args)+))
    };
    (~internal ($($func: tt)+) $($args: tt)+) => {
        std::iter::once(lambda!($($func)+)).chain(lambda!(~internal $($args)+))
    };
    (~internal $($args: tt)+) => {
        std::iter::once(lambda!($($args)+))
    };
}