use std::mem;

use crate::term::reduce::BetaReduce;
use crate::term::reduce::LocalNamelessTerm;
use crate::term::reduce::Var;

pub struct Normal;

impl<T: Clone> BetaReduce<Var<T>> for Normal {
    fn beta_reduce_step(term: &mut LocalNamelessTerm<T>) -> bool {
        match term {
            LocalNamelessTerm::Var(_) => false,
            LocalNamelessTerm::Abs(_, body) => Self::beta_reduce_step(body),
            LocalNamelessTerm::App(func, arg) => match func.as_mut() {
                LocalNamelessTerm::Abs(_, body) => {
                    Self::beta_reduce_step(body);
                    body.open(0, arg);
                    *term = mem::replace(body, LocalNamelessTerm::Var(Var::Bound(0)));
                    true
                },
                func => {
                    let func_reduced = Self::beta_reduce_step(func);
                    let arg_reduced = Self::beta_reduce_step(arg);
                    func_reduced || arg_reduced
                },
            },
        }
    }
}