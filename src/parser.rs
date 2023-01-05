use chumsky::prelude::*;

use crate::lexer::Token;
use crate::statement::Statement;
use crate::term::Term;

fn statement_parser<'s>() -> impl Parser<Token<'s>, Statement<'s>, Error = Simple<Token<'s>>> {
    ident_parser().then_ignore(just(Token::Equals).then_ignore(filler_parser()))
        .then(term_parser())
        .then_ignore(just(Token::Dot).then_ignore(filler_parser()))
        .map(|(name, term)| Statement::Bind(name, term))
}

fn term_parser<'s>() -> impl Parser<Token<'s>, Term<'s>, Error = Simple<Token<'s>>> {
    recursive(|term| {
        let var = ident_parser().map(|ident| Term::var(ident))
            .labelled("variable");

        let abs = just(Token::Lambda).then_ignore(filler_parser())
            .ignore_then(ident_parser())
            .then_ignore(just(Token::Dot)
                .then_ignore(filler_parser()))
            .then(term.clone())
            .map(|(param, body)| Term::abs(param, body));

        let parens = term.clone()
            .delimited_by(just(Token::OpenParens).then_ignore(filler_parser()), just(Token::CloseParens).then_ignore(filler_parser()));

        let app = parens.clone()
            .or(var.clone())
            .then(abs.clone()
                .or(var.clone())
                .or(parens.clone())
                .repeated()
                .at_least(1))
            .foldl(|func, arg| Term::app(func, arg));

        abs.or(app)
            .or(var)
            .or(parens)
    })
}

fn ident_parser<'s>() -> impl Parser<Token<'s>, &'s str, Error = Simple<Token<'s>>> + Clone {
    let ident = select! {
        Token::Ident(ident) => ident,
    };
    ident.then_ignore(filler_parser())
        .labelled("identifier")
}

fn filler_parser<'s>() -> impl Parser<Token<'s>, Vec<Token<'s>>, Error = Simple<Token<'s>>> + Clone {
    just(Token::Whitespace)
        .or(filter(|token| matches!(token, Token::LineComment(_))))
        .repeated()
}