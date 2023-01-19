use crate::Term;
use crate::repl::Statement;

#[derive(Clone)]
pub enum Command<T> {
    Reduce(Term<T>),
    Exec(Vec<Statement<T>>),
    Display(T),
    Debug(T),
    Exit,
}