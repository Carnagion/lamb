use std::collections::HashMap;
use std::io;
use std::io::Error as IoError;
use std::ops::Range;

use ariadne::Color;
use ariadne::ColorGenerator;
use ariadne::Fmt;
use ariadne::Label;
use ariadne::Report;
use ariadne::ReportKind;
use ariadne::Source;

use chumsky::Stream;
use chumsky::prelude::*;

use logos::Logos;

use lambda::Normal;
use lambda::repl::Command;
use lambda::repl::Statement;
use lambda::repl::lexer::Token;
use lambda::repl::parser::*;

const REPORT_KIND_INFO: ReportKind = ReportKind::Custom("Info", Color::Green);

fn main() {
    let reduce_limit = 1000;
    let mut binds = HashMap::new();

    loop {
        let mut color_gen = ColorGenerator::new();

        let mut source = String::new();
        if let Err(error) = io::stdin().read_line(&mut source) {
            report_read_error(&source, error);
            continue;
        }
        
        let lex_result = Token::lexer(&source).spanned();
        let parse_result = filler_parser().ignore_then(command_parser())
            .then_ignore(end())
            .parse(Stream::from_iter(source.len() - 1..source.len(), lex_result));
        let command = match parse_result {
            Ok(command) => command,
            Err(errors) => {
                report_syntax_error(&source, errors, color_gen.next());
                continue;
            },
        };
        
        match command {
            Command::Reduce(term) => {
                let reduced = term.beta_reduced_limit::<Normal>(reduce_limit);
                let count = reduced.count();
                report_term_reduced(&source, count);
                println!("{}", reduced.term());
                if count >= reduce_limit {
                    report_reduce_limit_reached(&source, reduce_limit, color_gen.next());
                }
            },
            Command::Exec(statements) => for statement in statements {
                match statement {
                    Statement::Bind(name, term) => {
                        let inserted = binds.insert(name.clone(), term);
                        let color = color_gen.next();
                        report_binding_added(&source, &name, color);
                        if inserted.is_some() {
                            report_binding_overwritten(&source, &name, color);
                        }
                    },
                }
            },
        }
    }
}

fn report_read_error(source: impl AsRef<str>, error: IoError) {
    Report::<Range<usize>>::build(ReportKind::Error, (), 0)
        .with_message(format!("{}", error))
        .finish()
        .print(Source::from(source))
        .unwrap();
}

fn report_syntax_error(source: impl AsRef<str>, errors: Vec<Simple<Token>>, color: Color) {
    errors.into_iter()
        .fold(Report::build(ReportKind::Error, (), 0)
            .with_message("Invalid syntax"), |report, error| report.with_label(Label::new(into_char_span(error.span(), &source))
                .with_message(format!("{}", error))
                .with_color(color)))
        .finish()
        .eprint(Source::from(source))
        .unwrap();
}

fn report_term_reduced(source: impl AsRef<str>, count: usize) {
    Report::<Range<usize>>::build(REPORT_KIND_INFO, (), 0)
        .with_message(format!("Reduced {} times", count.fg(Color::Green)))
        .finish()
        .print(Source::from(source))
        .unwrap();
}

fn report_reduce_limit_reached(source: impl AsRef<str>, reduce_limit: usize, color: Color) {
    Report::build(ReportKind::Warning, (), 0)
        .with_message("Reduction limit reached")
        .with_label(Label::new(0..source.as_ref().chars().count())
            .with_message("possibly divergent term")
            .with_color(color))
        .with_note(format!("current reduction limit is {}", reduce_limit.fg(color)))
        .finish()
        .print(Source::from(source))
        .unwrap();
}

fn report_binding_added(source: impl AsRef<str>, name: impl AsRef<str>, color: Color) {
    Report::<Range<usize>>::build(REPORT_KIND_INFO, (), 0)
        .with_message(format!("Binding {} added", name.as_ref().fg(color)))
        .finish()
        .print(Source::from(source))
        .unwrap();
}

fn report_binding_overwritten(source: impl AsRef<str>, name: impl AsRef<str>, color: Color) {
    Report::<Range<usize>>::build(ReportKind::Warning, (), 0)
        .with_message(format!("Binding {} overwritten", name.as_ref().fg(color)))
        .finish()
        .print(Source::from(source))
        .unwrap();
}

fn into_char_span(byte_span: Range<usize>, source: impl AsRef<str>) -> Range<usize> {
    let source = source.as_ref();
    into_char_index(byte_span.start(), source)..into_char_index(byte_span.end(), source)
}

fn into_char_index(byte_index: usize, source: impl AsRef<str>) -> usize {
    let source = source.as_ref();
    let mut bytes = 0;
    source.chars()
        .enumerate()
        .find_map(|(index, char)| {
            bytes += char.len_utf8();
            if byte_index < bytes {
                Some(index)
            } else {
                None
            }
        })
        .unwrap_or(source.chars().count())
}