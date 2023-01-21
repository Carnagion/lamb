use std::io;
use std::io::Error as IoError;
use std::io::Write;
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

use lambda::repl::CommandOutcome;
use lambda::repl::Repl;
use lambda::repl::lexer::Token;
use lambda::repl::parser::*;

const REPORT_KIND_INFO: ReportKind = ReportKind::Custom("Info", Color::Green);

fn main() -> Result<(), IoError> {
    let mut repl = Repl::new();
    let mut color_gen = ColorGenerator::new();

    'repl: loop {
        print!("λ> ");
        io::stdout().flush()?;

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
                report_syntax_error(&source, errors, &mut color_gen)?;
                continue;
            },
        };
        
        for action in repl.exec(command) {
            match action {
                CommandOutcome::TermReduced(reduced) => {
                    report_term_reduced(&source, reduced.count)?;
                    println!("{}", reduced.term);
                },
                CommandOutcome::ReduceLimitReached(limit) => report_reduce_limit_reached(&source, limit, color_gen.next())?,
                CommandOutcome::BindAdded(name) => report_binding_added(&source, name, color_gen.next())?,
                CommandOutcome::BindOverwritten(name) => {
                    let color = color_gen.next();
                    report_binding_added(&source, &name, color)?;
                    report_binding_overwritten(&source, &name, color)?;
                },
                CommandOutcome::ReduceLimitSet(limit) => report_limit_set(&source, limit, color_gen.next())?,
                CommandOutcome::DisplayReduceLimit(limit) => report_reduce_limit(&source, limit, color_gen.next())?,
                CommandOutcome::Exit => break 'repl,
            }
        }
    }

    Ok(())
}

fn report_read_error(source: impl AsRef<str>, error: IoError) {
    Report::<Range<usize>>::build(ReportKind::Error, (), 0)
        .with_message(format!("{}", error))
        .finish()
        .print(Source::from(source))
        .unwrap();
}

fn report_syntax_error(source: impl AsRef<str>, errors: Vec<Simple<Token>>, color_gen: &mut ColorGenerator) -> Result<(), IoError> {
    errors.into_iter()
        .fold(Report::build(ReportKind::Error, (), 0)
            .with_message("Invalid syntax"), |report, error| {
                let color = color_gen.next();
                report.with_label(Label::new(into_char_span(error.span(), &source))
                    .with_message(format!("{}", error.fg(color)))
                    .with_color(color))
            })
        .finish()
        .eprint(Source::from(source))
}

fn report_term_reduced(source: impl AsRef<str>, count: usize) -> Result<(), IoError> {
    Report::<Range<usize>>::build(REPORT_KIND_INFO, (), 0)
        .with_message(format!("Reduced {} times", count.fg(Color::Green)))
        .finish()
        .print(Source::from(source))
}

fn report_reduce_limit_reached(source: impl AsRef<str>, reduce_limit: usize, color: Color) -> Result<(), IoError> {
    Report::build(ReportKind::Warning, (), 0)
        .with_message("Reduction limit reached")
        .with_label(Label::new(0..source.as_ref().chars().count() - 1)
            .with_message("possibly divergent term")
            .with_color(color))
        .with_note(format!("current reduction limit is {}", reduce_limit.fg(color)))
        .finish()
        .print(Source::from(source))
}

fn report_binding_added(source: impl AsRef<str>, name: impl AsRef<str>, color: Color) -> Result<(), IoError> {
    Report::<Range<usize>>::build(REPORT_KIND_INFO, (), 0)
        .with_message(format!("Binding {} added", name.as_ref().fg(color)))
        .finish()
        .print(Source::from(source))
}

fn report_binding_overwritten(source: impl AsRef<str>, name: impl AsRef<str>, color: Color) -> Result<(), IoError> {
    Report::<Range<usize>>::build(ReportKind::Warning, (), 0)
        .with_message(format!("Binding {} overwritten", name.as_ref().fg(color)))
        .finish()
        .print(Source::from(source))
}

fn report_limit_set(source: impl AsRef<str>, reduce_limit: usize, color: Color) -> Result<(), IoError> {
    Report::<Range<usize>>::build(REPORT_KIND_INFO, (), 0)
        .with_message(format!("Reduction limit set to {}", reduce_limit.fg(color)))
        .finish()
        .print(Source::from(source))
}

fn report_reduce_limit(source: impl AsRef<str>, reduce_limit: usize, color: Color) -> Result<(), IoError> {
    Report::<Range<usize>>::build(REPORT_KIND_INFO, (), 0)
        .with_message(format!("Current reduction limit is {}", reduce_limit.fg(color)))
        .finish()
        .print(Source::from(source))
}

fn into_char_span(byte_span: Range<usize>, source: impl AsRef<str>) -> Range<usize> {
    let source = source.as_ref();
    source[..byte_span.start()].chars().count()..source[..byte_span.end()].chars().count()
}