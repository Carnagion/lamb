use crate::Term;
use crate::repl::Statement;

pub enum Command<T> {
    Reduce(Term<T>),
    Exec(Vec<Statement<T>>),
}