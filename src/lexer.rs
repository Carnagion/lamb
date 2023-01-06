use logos;
use logos::Lexer;
use logos::Logos;

#[derive(Clone, Debug, Eq, Hash, Logos, PartialEq)]
pub enum Token<'s> {
    #[token("λ")]
    Lambda,
    #[regex(r"[^λ\.()\s=#]+")]
    Ident(&'s str),
    #[token(".")]
    Dot,
    #[token("(")]
    OpenParens,
    #[token(")")]
    CloseParens,
    #[token("=")]
    Equals,
    #[regex(r"\s+")]
    Whitespace,
    #[regex("#.*", Token::line_comment)]
    LineComment(&'s str),
    #[error]
    Unknown,
}

impl Token<'_> {
    fn line_comment<'s>(lexer: &Lexer<'s, Token<'s>>) -> &'s str {
        &lexer.slice()[1..]
    }
}