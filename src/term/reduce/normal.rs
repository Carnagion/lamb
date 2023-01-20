//! The normal-order β-reduction strategy.

use std::mem;

use crate::BetaReduce;
use crate::LocalNamelessTerm;
use crate::Var;

/// The normal-order β-reduction strategy for [Term](crate::Term)s.
/// 
/// This strategy reduces the leftmost, outermost β-redexes first.
pub struct Normal;

impl<T: Clone> BetaReduce<Var<T>> for Normal {
    fn beta_reduce_step(&self, term: &mut LocalNamelessTerm<T>) -> bool {
        match term {
            LocalNamelessTerm::Var(_) => false,
            LocalNamelessTerm::Abs(_, body) => self.beta_reduce_step(body),
            LocalNamelessTerm::App(func, arg) => match func.as_mut() {
                LocalNamelessTerm::Abs(_, body) => {
                    self.beta_reduce_step(body);
                    body.open(0, arg);
                    // The body is replaced with a dummy value but the entire abstraction ceases to exist and cannot be accessed again, so this is ok
                    *term = mem::replace(body, LocalNamelessTerm::var(Var::Bound(0)));
                    true
                },
                func => {
                    let func_reduced = self.beta_reduce_step(func);
                    let arg_reduced = self.beta_reduce_step(arg);
                    func_reduced || arg_reduced
                },
            },
        }
    }
}