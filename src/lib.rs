//! Pure untyped lambda calculus in safe Rust.
//! 
//! # Terms
//! 
//! At the core of this crate is the [Term] type, which represents an untyped lambda calculus term.
//! A [Term] can be either a variable, an abstraction, or an application.
//! 
//! [Term]s are generic over their identifier type - meaning variables and formal parameters are not constrained to being any one particular type such as strings.
//! 
//! *Note: Most operations on [Term]s require the identifier type to `impl`ement [Clone] and [Eq], but this is a trivial task.*
//! 
//! # Constructing terms
//! 
//! [Term]s can be constructed within Rust code in many ways.
//! 
//! The simplest (and arguably most elegant) way is the [lambda!] macro, offering syntax that very closely resembles standard untyped lambda calculus notation.
//! It supports syntax sugar for multiple parameters as well as multiple argument application, making it the most convenient way to construct [Term]s.
//! ```
//! use lambda::lambda;
//! 
//! let id = lambda!(λ x. x);
//! let succ = lambda!(λ n s z. s (n s z));
//! let z = lambda!(λ f. (λ x. f (λ y. x x y)) (λ x. f (λ y. x x y)));
//! ```
//! 
//! Also provided are the [var!], [abs!], and [app!] macros, which support the same syntax sugar, but mainly operate on expressions rather than raw tokens.
//! This makes them a better choice for working with pre-defined [Term]s, such as those in the [prelude], while still maintaining readability.
//! ```
//! use lambda::abs;
//! use lambda::app;
//! use lamdba::lambda;
//! 
//! let inner = lambda!(λ x. f (λ y. x x y));
//! let z = abs!(f. app!(inner.clone(), inner));
//! ```
//! 
//! In case these do not suffice, [Term]'s enum variants and associated functions offer finer control over their construction, at the cost of readability.
//! 
//! # Reducing terms
//! 
//! The primary method of evaluating [Term]s is β-reduction, which is handled by the [BetaReduce] trait.
//! It allows implementing different strategies for β-reducing [Term]s, making this crate highly extendable.
//! ```
//! use lambda::lambda;
//! use lambda::Normal;
//! 
//! let term = lambda!((λ x. z) ((λ w. w w w) (λ w. w w w)));
//! let reduced = term.reduced::<Normal>();
//! ```
//! 
//! The default `impl`ementations of [BetaReduce] rely on locally nameless representations of [Term]s in order to safely reduce them without needing to α-convert identifiers.
//! [LocalNamelessTerm]s can be β-reduced in-place, whereas regular [Term]s must first be converted to [LocalNamelessTerm]s for β-reduction.
//! ```
//! use lambda::lambda;
//! use lambda::LocalNamelessTerm;
//! use lambda::Normal;
//! 
//! let term = lambda!((λ x. z) ((λ w. w w w) (λ w. w. w w)));
//! let local_nameless_term = LocalNamelessTerm::from(&term);
//! local_nameless_term.reduce::<Normal>();
//! ```
//! 
//! *Note: Converting a [Term] to a [LocalNamelessTerm] also does not consume the original [Term], leaving it available for further use if necessary.*

mod lexer;
mod parser;

#[macro_use]
pub mod term;
pub use term::*;

pub mod statement;
pub use statement::*;

pub mod prelude;