use crate::term::Term;

/// The function composition combinator `B`.
///
/// Alpha-equivalent to the term `λf g x. f (g x)`.
pub fn compose() -> Term<&'static str> {
    lambda!(λ f g x. f (g x))
}

/// The argument flipping combinator `C`.
///
/// Alpha-equivalent to the term `λf x y. f y x`.
pub fn flip() -> Term<&'static str> {
    lambda!(λ f x y. f y x)
}

/// The identity combinator `I`.
///
/// Alpha-equivalent to the term `λx. x`.
pub fn id() -> Term<&'static str> {
    lambda!(λ x. x)
}

/// The constant or discarding combinator `K`.
///
/// Alpha-equivalent to the term `λx y. x`.
pub fn constant() -> Term<&'static str> {
    lambda!(λ x y. x)
}

/// The diverging combinator `Ω`.
///
/// Alpha-equivalent to the term `ω ω`, where `ω` is the self-applying combinator (see [app_self]).
pub fn omega() -> Term<&'static str> {
    app!(app_self(), app_self())
}

/// The reverse application combinator `R`.
///
/// Alpha-equivalent to the term `λx y. y x`.
pub fn app_rev() -> Term<&'static str> {
    lambda!(λ x y. y x)
}

/// The substitution combinator `S`.
///
/// Alpha-equivalent to the term `λx y z. x z (y z)`.
pub fn sub() -> Term<&'static str> {
    lambda!(λ x y z. x z (y z))
}

/// Turing's fixed-point combinator `Θ`.
///
/// Alpha equivalent to the term `(λx y. y (x x y)) (λx y. y (x x y))`.
pub fn fix_turing() -> Term<&'static str> {
    lambda!((λ x y. y (x x y)) (λ x y. y (x x y)))
}

/// The duplicating combinator `W`.
///
/// Alpha-equivalent to the term `λf x. f x x`.
pub fn dup() -> Term<&'static str> {
    lambda!(λ f x. f x x)
}

/// The lazy fixed-point combinator `Y`.
///
/// Alpha-equivalent to the term `λf. (λx. f (x x)) (λx. f (x x))`.
pub fn fix_lazy() -> Term<&'static str> {
    lambda!(λ f. (λ x. f (x x)) (λ x. f (x x)))
}

/// The strict fixed-point combinator `Z`.
///
/// Alpha-equivalent to the term `λf. (λx. f (λy. x x y)) (λx. f (λy. x x y))`.
pub fn fix_strict() -> Term<&'static str> {
    lambda!(λ f. (λ x. f (λ y. x x y)) (λ x. f (λ y. x x y)))
}

/// The universal combinator `i`.
///
/// Alpha-equivalent to the term `λx. x S K`, where `S` is the substitution combinator (see [sub]) and `K` is the constant combinator (see [constant]).
pub fn universal() -> Term<&'static str> {
    abs!(x. app!(var!(x), sub(), constant()))
}

/// The self-application combinator `ω`.
///
/// Alpha-equivalent to the term `λx. x x`.
pub fn app_self() -> Term<&'static str> {
    lambda!(λ x. x x)
}