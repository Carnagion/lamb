//! [Repl] (read-eval-print-loop) functions for executing [Command]s and handling the results.

use std::collections::HashMap;
use std::hash::Hash;

use crate::LocalNamelessTerm;
use crate::Normal;
use crate::ReducedTerm;
use crate::Term;

pub mod statement;
pub use statement::*;

pub mod lexer;

pub mod parser;

/// A command that can be executed by a [Repl].
/// 
/// Certain [Command]s (such as executing a [Statement]) can modify the [Repl]'s state.
#[derive(Clone)]
pub enum Command<T> {
    /// β-reduce a [Term].
    /// 
    /// Since this has the potential to cause a stack overflow due to deep recursion, the [Repl] limits the number of β-reduction steps that can be performed (see [Term::beta_reduced_limit]).
    /// However, the limit can be modified.
    Reduce(Term<T>),
    /// Execute one or more [Statement]s, updating the [Repl]'s state as necessary.
    Exec(Vec<Statement<T>>),
    /// Get the [Repl]'s β-reduction limit.
    GetReduceLimit,
    /// Set the [Repl]'s β-reduction limit.
    SetReduceLimit(usize),
    /// Exit the [Repl].
    Exit,
}

/// The outcome of a [Repl] executing a [Command].
/// 
/// Executing a [Command] can have one or more [CommandOutcome]s, as certain situations are considered warnings by the [Repl], even if the [Command] was executed successfully.
pub enum CommandOutcome<T> {
    /// A [Term] was reduced upto the (implied) β-reduction limit.
    TermReduced(ReducedTerm<T>),
    /// The β-reduction limit was reached while β-reducing a [Term].
    /// 
    /// This is considered a warning by the [Repl].
    ReduceLimitReached(usize),
    /// A [Term] with a specific name was added.
    /// 
    /// Future execution of [Command::Reduce]s will have the [Term]s' free variables replaced with their matching bindings (if they exist) before β-reduction.
    BindAdded(T),
    /// A [Term] with a specific name was added (see [CommandOutcome::BindAdded]), but a previous [Term] with the same name existed and was overwritten.
    /// 
    /// This is considered a warning by the [Repl].
    BindOverwritten(T),
    /// The [Repl]'s β-reduction limit was retrieved.
    ReduceLimitGot(usize),
    /// The [Repl]'s β-reduction limit was updated.
    ReduceLimitSet(usize),
    /// The [Repl] must be exited.
    Exit,
}

/// A read-eval-print-loop that can execute [Command]s.
pub struct Repl<T> {
    binds: HashMap<T, LocalNamelessTerm<T>>,
    reduce_limit: usize,
}

impl<T> Repl<T> {
    /// Creates a new [Repl] with no bindings and the default β-reduction limit.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Clone + Eq + Hash> Repl<T> {
    /// Executes a [Command] on the [Repl].
    /// 
    /// The resulting [Vec] will always have at least one [CommandOutcome].
    pub fn exec(&mut self, command: Command<T>) -> Vec<CommandOutcome<T>> {
        let mut actions = Vec::with_capacity(1);
        match command {
            Command::Reduce(term) => {
                let mut local_nameless = LocalNamelessTerm::from(&term);
                local_nameless.rebind(&mut self.binds);
                let count = local_nameless.beta_reduce_limit(self.reduce_limit, &Normal);
                actions.push(CommandOutcome::TermReduced(ReducedTerm {
                    count,
                    term: Term::try_from(&local_nameless).unwrap(),
                }));
                if count >= self.reduce_limit {
                    actions.push(CommandOutcome::ReduceLimitReached(count));
                }
            },
            Command::Exec(statements) => actions.extend(statements.into_iter()
                .map(|statement| match statement {
                    Statement::Bind(name, term) => {
                        let mut local_nameless = LocalNamelessTerm::from(&term);
                        local_nameless.rebind(&mut self.binds);
                        match self.binds.insert(name.clone(), local_nameless) {
                            None => CommandOutcome::BindAdded(name),
                            Some(_) => CommandOutcome::BindOverwritten(name),
                        }
                    },
                })),
            Command::GetReduceLimit => actions.push(CommandOutcome::ReduceLimitGot(self.reduce_limit)),
            Command::SetReduceLimit(limit) => {
                self.reduce_limit = limit;
                actions.push(CommandOutcome::ReduceLimitSet(limit));
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