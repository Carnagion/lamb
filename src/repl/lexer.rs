use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use logos::Lexer;
use logos::Logos;

#[derive(Clone, Debug, Eq, Hash, Logos, PartialEq)]
pub enum Token<'s> {
    #[token("λ")]
    Lambda,
    #[regex(r"[^λ\.()\s=;#]+")]
    Ident(&'s str),
    #[token(".")]
    Dot,
    #[token("(")]
    OpenParens,
    #[token(")")]
    CloseParens,
    #[regex(r"\s+")]
    Whitespace,
    #[token("=")]
    Equals,
    #[token(";")]
    Semicolon,
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

impl Display for Token<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let str = match self {
            Self::Lambda => "λ",
            Self::Ident(ident) => ident,
            Self::Dot => ".",
            Self::OpenParens => "(",
            Self::CloseParens => ")",
            Self::Whitespace => "whitespace",
            Self::Equals => "=",
            Self::Semicolon => ";",
            Self::LineComment(_) => "comment",
            Self::Unknown => "unknown",
        };
        write!(formatter, "{}", str)
    }
}