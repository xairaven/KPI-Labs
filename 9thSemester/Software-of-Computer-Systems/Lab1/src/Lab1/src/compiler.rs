use crate::compiler::syntax::{SyntaxAnalyzer, SyntaxError};
use crate::utils::StringExtension;
use colored::Colorize;
use std::ops::Add;

pub fn compile(source: &str, is_pretty: bool) -> String {
    let tokens = tokenizer::tokenize(source);
    let syntax_errors = SyntaxAnalyzer::new(tokens).analyze();

    report(source, syntax_errors, is_pretty)
}

fn report(source: &str, syntax_errors: Vec<SyntaxError>, is_pretty: bool) -> String {
    let mut result = String::new();

    let first_line = match syntax_errors.len() {
        0 => format!("{}: {}\n", "Analysis result".bold(), "OK!".bold().green()),
        n => {
            format!(
                "{}: Found {} {}.\n",
                "Analysis result".bold(),
                n.to_string().red(),
                "errors".red()
            )
        },
    };
    result.push_str(&first_line);

    result.push_str(&format!("\n{}:\n", "Code".bold().yellow()));
    result.push_str(&format!("{}\n", source.replace("\n", " ")));

    if !syntax_errors.is_empty() {
        let errors = match is_pretty {
            true => format_errors_pretty(source, syntax_errors),
            false => format_errors(syntax_errors),
        };
        result.push_str(&format!("{}\n", errors));
    }

    result
}

fn format_errors_pretty(source: &str, syntax_errors: Vec<SyntaxError>) -> String {
    let mut result = String::new();

    // First line: Underlines
    let length = source.len();
    let mut first_line = " ".repeat(length).add("\n");
    for error in &syntax_errors {
        let underline_length = error.token.position.end - error.token.position.start;
        if underline_length == 1 {
            first_line.replace_char(error.token.position.start, '^');
        } else {
            for index in (error.token.position.start + 1)..(error.token.position.end - 1)
            {
                first_line.replace_char(index, '-');
            }

            first_line.replace_char(error.token.position.start, '^');
            first_line.replace_char(error.token.position.end - 1, '^');
        }
    }
    result.push_str(&first_line);

    let biggest_error_length = syntax_errors
        .iter()
        .map(|error| error.to_string().len())
        .max()
        .unwrap_or(0)
        + 1;

    // Other lines
    for error in syntax_errors.iter().rev() {
        // One for -, another one for \n
        let mut line = " ".repeat(length + 2);
        for error in syntax_errors.iter() {
            line.replace_char(error.token.position.start, '|');
        }
        for index in (error.token.position.start + 1)..(length + 1) {
            line.replace_char(index, '_');
        }
        line.push_str(&format!("{}\n", error.display(biggest_error_length)));
        result.push_str(&line);
    }

    result
}

fn format_errors(syntax_errors: Vec<SyntaxError>) -> String {
    let mut result = String::new();

    let biggest_error_length = syntax_errors
        .iter()
        .map(|error| error.to_string().len())
        .max()
        .unwrap_or(0)
        + 1;

    for error in syntax_errors {
        result.push_str(&format!("{}\n", error.display(biggest_error_length)));
    }

    result
}

pub mod syntax;
pub mod tokenizer;
