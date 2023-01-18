use crate::ReducedTerm;
use crate::Term;
use crate::repl::Statement;

pub enum CommandOutcome<T> {
    StatementExecuted,
    BindingOverwritten(T),
    TermReduced(ReducedTerm<T>),
    ReductionLimitReached(usize),
}

pub enum Command<T> {
    Reduce(Term<T>),
    Exec(Vec<Statement<T>>),
}