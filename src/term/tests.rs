use crate::abs;
use crate::app;
use crate::var;
use crate::prelude::combinators;

#[test]
fn display_compose() {
    assert_eq!(format!("{}", combinators::compose()), "λf. λg. λx. f (g x)");
}

#[test]
fn eq_compose() {
    assert_eq!(combinators::compose(), abs!(f g x. app!(var!(f), app!(var!(g), var!(x)))));
}

#[test]
fn display_flip() {
    assert_eq!(format!("{}", combinators::flip()), "λf. λx. λy. f y x");
}

#[test]
fn eq_flip() {
    assert_eq!(combinators::flip(), abs!(f x y. app!(var!(f), var!(y), var!(x))));
}

#[test]
fn display_id() {
    assert_eq!(format!("{}", combinators::id()), "λx. x");
}

#[test]
fn eq_id() {
    assert_eq!(combinators::id(), abs!(x. var!(x)));
}

#[test]
fn display_constant() {
    assert_eq!(format!("{}", combinators::constant()), "λx. λy. x");
}

#[test]
fn eq_constant() {
    assert_eq!(combinators::constant(), abs!(x y. var!(x)));
}

#[test]
fn display_omega() {
    assert_eq!(format!("{}", combinators::omega()), "(λx. x x) (λx. x x)");
}

#[test]
fn eq_omega() {
    assert_eq!(combinators::omega(), app!(abs!(x. app!(var!(x), var!(x))), abs!(x. app!(var!(x), var!(x)))));
}

#[test]
fn display_app_rev() {
    assert_eq!(format!("{}", combinators::app_rev()), "λx. λy. y x");
}

#[test]
fn eq_app_rev() {
    assert_eq!(combinators::app_rev(), abs!(x y. app!(var!(y), var!(x))));
}

#[test]
fn display_sub() {
    assert_eq!(format!("{}", combinators::sub()), "λx. λy. λz. x z (y z)");
}

#[test]
fn eq_sub() {
    assert_eq!(combinators::sub(), abs!(x y z. app!(var!(x), var!(z), app!(var!(y), var!(z)))));
}

#[test]
fn display_fix_turing() {
    assert_eq!(format!("{}", combinators::fix_turing()), "(λx. λy. y (x x y)) (λx. λy. y (x x y))");
}

#[test]
fn eq_fix_turing() {
    assert_eq!(combinators::fix_turing(), app!(abs!(x y. app!(var!(y), app!(var!(x), var!(x), var!(y)))), abs!(x y. app!(var!(y), app!(var!(x), var!(x), var!(y))))));
}

#[test]
fn display_dup() {
    assert_eq!(format!("{}", combinators::dup()), "λf. λx. f x x");
}

#[test]
fn eq_dup() {
    assert_eq!(combinators::dup(), abs!(f x. app!(var!(f), var!(x), var!(x))));
}

#[test]
fn display_fix_lazy() {
    assert_eq!(format!("{}", combinators::fix_lazy()), "λf. (λx. f (x x)) (λx. f (x x))");
}

#[test]
fn eq_fix_lazy() {
    assert_eq!(combinators::fix_lazy(), abs!(f. app!(abs!(x. app!(var!(f), app!(var!(x), var!(x)))), abs!(x. app!(var!(f), app!(var!(x), var!(x)))))));
}

#[test]
fn display_fix_strict() {
    assert_eq!(format!("{}", combinators::fix_strict()), "λf. (λx. f (λy. x x y)) (λx. f (λy. x x y))");
}

#[test]
fn eq_fix_strict() {
    assert_eq!(combinators::fix_strict(), abs!(f. app!(abs!(x. app!(var!(f), abs!(y. app!(var!(x), var!(x), var!(y))))), abs!(x. app!(var!(f), abs!(y. app!(var!(x), var!(x), var!(y))))))));
}

#[test]
fn display_universal() {
    assert_eq!(format!("{}", combinators::universal()), "λx. x (λx. λy. λz. x z (y z)) (λx. λy. x)");
}

#[test]
fn eq_universal() {
    assert_eq!(combinators::universal(), abs!(x. app!(var!(x), abs!(x y z. app!(var!(x), var!(z), app!(var!(y), var!(z)))), abs!(x y. var!(x)))));
}

#[test]
fn display_app_self() {
    assert_eq!(format!("{}", combinators::app_self()), "λx. x x");
}

#[test]
fn eq_app_self() {
    assert_eq!(combinators::app_self(), abs!(x. app!(var!(x), var!(x))));
}