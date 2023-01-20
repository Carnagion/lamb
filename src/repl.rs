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

pub enum CommandOutcome<T> {
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
    pub fn exec(&mut self, command: Command<T>) -> Vec<CommandOutcome<T>> {
        let mut actions = Vec::with_capacity(1);
        match command {
            Command::Reduce(term) => {
                let reduced = term.beta_reduced_limit::<Normal>(self.reduce_limit);
                let count = reduced.count();
                actions.push(CommandOutcome::TermReduced(reduced));
                if count >= self.reduce_limit {
                    actions.push(CommandOutcome::ReduceLimitReached(count));
                }
            },
            Command::Exec(statements) => actions.extend(statements.into_iter()
                .map(|statement| match statement {
                    Statement::Bind(name, term) => match self.binds.insert(name.clone(), term) {
                        None => CommandOutcome::BindAdded(name),
                        Some(_) => CommandOutcome::BindOverwritten(name),
                    },
                })),
            Command::Limit(limit) => match limit {
                None => actions.push(CommandOutcome::DisplayReduceLimit(self.reduce_limit)),
                Some(limit) => {
                    self.reduce_limit = limit;
                    actions.push(CommandOutcome::ReduceLimitSet(limit));
                },
            },
            Command::Exit => actions.push(CommandOutcome::Exit),
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