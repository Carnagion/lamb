//! Church-encoded boolean [Term]s and operations.

use crate::Term;

/// The Church-encoded boolean `false`.
/// 
/// This is α-equivalent to `λt f. f`.
pub fn fls() -> Term<&'static str> {
    lambda!(λ t f. f)
}

/// The Church-encoded boolean value `true`.
/// 
/// This is α-equivalent to `λt f. t`.
pub fn tru() -> Term<&'static str> {
    lambda!(λ t f. t)
}

/// The `if-then-else` function.
/// 
/// This is α-equivalent to `λc t e. c t e`.
/// Since Church booleans encode this functionality in themselves, this is technically redundant.
pub fn if_then_else() -> Term<&'static str> {
    lambda!(λ c t e. c t e)
}

/// The boolean `not` function.
/// 
/// This is α-equivalent to `λb. if-then-else b fls tru` (see [fls], [tru], and [if_then_else]).
pub fn not() -> Term<&'static str> {
    abs!(b. app!(if_then_else(), var!(b), fls(), tru()))
}

/// The boolean `and` function.
/// 
/// This is α-equivalent to `λl r. l r fls`, where `fls` is the Church-encoded boolean value `false` (see [fls]).
pub fn and() -> Term<&'static str> {
    abs!(l r. app!(var!(l), var!(r), fls()))
}

/// The boolean `or` function.
/// 
/// This is α-equivalent to `λl r. l tru r`, where `tru` is the Church-encoded boolean value `true` (see [tru]).
pub fn or() -> Term<&'static str> {
    abs!(l r. app!(var!(l), tru(), var!(r)))
}