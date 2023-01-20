use std::collections::HashMap;
use std::hash::Hash;

use crate::Normal;
use crate::ReducedTerm;
use crate::Term;

pub mod statement;
pub use statement::*;

pub mod lexer;

pub mod parser;

#[derive(Clone)]
pub enum Command<T> {
    Reduce(Term<T>),
    Exec(Vec<Statement<T>>),
    Limit(Option<usize>),
    Exit,
}

pub enum Action<T> {
    TermReduced(ReducedTerm<T>),
    ReduceLimitReached(usize),
    BindAdded(T),
    BindOverwritten(T),
    ReduceLimitSet(usize),
    DisplayReduceLimit(usize),
    Exit,
}

pub struct Repl<T> {
    binds: HashMap<T, Term<T>>,
    reduce_limit: usize,
}

impl<T> Repl<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Clone + Eq + Hash> Repl<T> {
    pub fn exec(&mut self, command: Command<T>) -> Vec<Action<T>> {
        let mut actions = Vec::with_capacity(1);
        match command {
            Command::Reduce(term) => {
                let reduced = term.beta_reduced_limit::<Normal>(self.reduce_limit);
                let count = reduced.count();
                actions.push(Action::TermReduced(reduced));
                if count >= self.reduce_limit {
                    actions.push(Action::ReduceLimitReached(count));
                }
            },
            Command::Exec(statements) => for statement in statements {
                match statement {
                    Statement::Bind(name, term) => actions.push(match self.binds.insert(name.clone(), term) {
                        None => Action::BindAdded(name),
                        Some(_) => Action::BindOverwritten(name),
                    }),
                }
            },
            Command::Limit(limit) => match limit {
                None => actions.push(Action::DisplayReduceLimit(self.reduce_limit)),
                Some(limit) => {
                    self.reduce_limit = limit;
                    actions.push(Action::ReduceLimitSet(limit));
                },
            },
            Command::Exit => actions.push(Action::Exit),
        }
        actions
    }
}

impl<T> Default for Repl<T> {
    fn default() -> Self {
        Self {
            binds: HashMap::default(),
            reduce_limit: 1000,
        }
    }
}