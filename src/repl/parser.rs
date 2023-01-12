use chumsky::prelude::*;

use crate::Statement;
use crate::Term;
use crate::repl::lexer::Token;

pub fn statement_parser<'s>() -> impl Parser<Token<'s>, Statement<&'s str>, Error = Simple<Token<'s>>> {
    ident_parser().then_ignore(just(Token::Equals).then_ignore(filler_parser()))
        .then(term_parser())
        .then_ignore(just(Token::Semicolon).then_ignore(filler_parser()))
        .map(|(name, term)| Statement::bind(name, term))
}

pub fn term_parser<'s>() -> impl Parser<Token<'s>, Term<&'s str>, Error = Simple<Token<'s>>> {
    recursive(|term| {
        let var = ident_parser().map(Term::var)
            .labelled("variable");

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
            .then(abs.clone()
                .or(var.clone())
                .or(parens.clone())
                .repeated()
                .at_least(1))
            .foldl(Term::app);

        abs.or(app)
            .or(var)
            .or(parens)
    })
}

pub fn ident_parser<'s>() -> impl Parser<Token<'s>, &'s str, Error = Simple<Token<'s>>> + Clone {
    let ident = select! {
        Token::Ident(ident) => ident,
    };
    ident.then_ignore(filler_parser())
        .labelled("identifier")
}

pub fn filler_parser<'s>() -> impl Parser<Token<'s>, Vec<Token<'s>>, Error = Simple<Token<'s>>> + Clone {
    just(Token::Whitespace)
        .or(filter(|token| matches!(token, Token::LineComment(_))))
        .repeated()
}