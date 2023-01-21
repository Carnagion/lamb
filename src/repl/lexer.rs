//! Lexer for lexing source code into [Token]s.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use logos::Lexer;
use logos::Logos;

/// Represents all the possible symbols or tokens in valid lambda calculus source code.
/// 
/// Due to the way [Logos] works, the [Token::Unknown] variant is necessary to describe an invalid [Token].
#[derive(Clone, Debug, Eq, Hash, Logos, PartialEq)]
pub enum Token<'s> {
    /// The lambda symbol (`λ`).
    /// 
    /// This can also be represented by a backslash (`\`).
    /// It is always displayed as a lambda when displaying [Term](crate::Term)s using their [Display] `impl`ementation.
    #[regex(r#"[λ\\]"#)]
    Lambda,
    /// An identifier conforming to the regular expression `[a-z][a-zA-Z\-]*`.
    #[regex(r"[a-z][a-zA-Z\-]*")]
    Ident(&'s str),
    /// A dot (`.`).
    #[token(".")]
    Dot,
    /// Opening parenthesis (`(`).
    #[token("(")]
    OpenParens,
    /// Closing parenthesis (`)`).
    #[token(")")]
    CloseParens,
    /// One or more whitespaces.
    /// 
    /// This includes regular spaces as well as tabs, newlines, carriage returns, etc.
    /// Whitespace is ignored by the parser, but is necessary in certain cases (such as to recognise something as multiple different [Token]s).
    #[regex(r"\s+")]
    Whitespace,
    /// The equals symbol (`=`).
    #[token("=")]
    Equals,
    /// A semicolon.
    #[token(";")]
    Semicolon,
    /// A line comment.
    /// 
    /// Line comments begin with a hash (`#`) and continue until a newline.
    /// Characters that are part of a line comment are ignored by the parser.
    #[regex("#.*", Token::line_comment)]
    LineComment(&'s str),
    /// A colon.
    #[token(":")]
    Colon,
    /// A non-negative integer (i.e. natural number, including zero).
    /// 
    /// If the number exceeds the bounds of [usize], a [Token::Unknown] is emitted instead.
    #[regex("[0-9]+", Token::number)]
    Number(usize),
    /// A [Token] that does not match any of the other valid variants.
    /// 
    /// This indicates some sort of syntax error.
    #[error]
    Unknown,
}

impl Token<'_> {
    fn line_comment<'s>(lexer: &Lexer<'s, Token<'s>>) -> &'s str {
        &lexer.slice()[1..]
    }

    fn number<'s>(lexer: &Lexer<'s, Token<'s>>) -> Option<usize> {
        lexer.slice()
            .parse()
            .ok()
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
            Self::Colon => ":",
            Self::Number(_) => "number",
            Self::Unknown => "unknown",
        };
        write!(formatter, "{}", str)
    }
}