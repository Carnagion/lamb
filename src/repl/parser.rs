use chumsky::prelude::*;

use crate::Term;
use crate::repl::Command;
use crate::repl::Statement;
use crate::repl::lexer::Token;

pub fn command_parser<'s>() -> impl Parser<Token<'s>, Command<String>, Error = Simple<Token<'s>>> {
    let exec = statement_parser().repeated()
        .at_least(1)
        .map(Command::Exec);

    let reduce = term_parser().map(Command::Reduce);

    let exit = just(Token::Ident("exit")).ignore_then(filler_parser())
        .to(Command::Exit);
    
    let limit = just(Token::Ident("limit")).ignore_then(filler_parser())
        .ignore_then(number_parser().or_not())
        .map(Command::Limit);
    
    exec.or(reduce)
        .or(just(Token::Colon).ignore_then(filler_parser())
            .ignore_then(exit.or(limit))
            .then_ignore(filler_parser()))
}

pub fn statement_parser<'s>() -> impl Parser<Token<'s>, Statement<String>, Error = Simple<Token<'s>>> {
    ident_parser().then_ignore(just(Token::Equals).then_ignore(filler_parser()))
        .then(term_parser())
        .then_ignore(just(Token::Semicolon).then_ignore(filler_parser()))
        .map(|(name, term)| Statement::bind(name, term))
}

pub fn term_parser<'s>() -> impl Parser<Token<'s>, Term<String>, Error = Simple<Token<'s>>> {
    recursive(|term| {
        let var = ident_parser().map(Term::var);

        let abs = just(Token::Lambda).then_ignore(filler_parser())
            .ignore_then(ident_parser().repeated())
            .then_ignore(just(Token::Dot)
                .then_ignore(filler_parser()))
            .then(term.clone())
            .foldr(Term::abs);

        let parens = term.clone()
            .delimited_by(just(Token::OpenParens).then_ignore(filler_parser()), just(Token::CloseParens).then_ignore(filler_parser()));

        let app = parens.clone()
            .or(var.clone())
            .then(choice((abs.clone(), var.clone(), parens.clone())).repeated()
                .at_least(1))
            .foldl(Term::app);

        choice((abs, app, var, parens))
    })
}

pub fn ident_parser<'s>() -> impl Parser<Token<'s>, String, Error = Simple<Token<'s>>> + Clone {
    let ident = select! {
        Token::Ident(ident) => ident.to_string(),
    };
    ident.then_ignore(filler_parser())
        .labelled("identifier")
}

pub fn number_parser<'s>() -> impl Parser<Token<'s>, usize, Error = Simple<Token<'s>>> + Clone {
    let number = select! {
        Token::Number(num) => num,
    };
    number.then_ignore(filler_parser())
        .labelled("number")
}

pub fn filler_parser<'s>() -> impl Parser<Token<'s>, Vec<Token<'s>>, Error = Simple<Token<'s>>> + Clone {
    just(Token::Whitespace)
        .or(filter(|token| matches!(token, Token::LineComment(_))))
        .repeated()
}