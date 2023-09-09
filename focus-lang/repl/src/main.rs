use std::io::{Error, Write};

use console::{style, Style, Term};
use interpreter::Interpreter;
use parser::{Parser, ParserError, ParserErrorInfo};
use scanner::{Scanner, Token};

fn main() -> Result<(), Error> {
    let mut term = Term::stdout();
    let mut interpreter = Interpreter::new();

    term.set_title("REPL");
    term.move_cursor_to(0, term.size().1 as usize)?;
    term.write_fmt(format_args!(
        "{} {}\n",
        style("focus-lang REPL").yellow(),
        style(env!("CARGO_PKG_VERSION")).yellow()
    ))?;
    term.write(b"> ")?;

    let mut history: Vec<String> = Vec::new();
    let mut history_index: usize = 0;

    let mut line = String::new();
    let mut expr = String::new();
    let mut cursor_x: usize = 0;
    loop {
        match term.read_key()? {
            console::Key::ArrowLeft => cursor_x = cursor_x.checked_sub(1).unwrap_or(0),
            console::Key::ArrowRight => cursor_x = (cursor_x + 1).min(line.len()),
            console::Key::ArrowUp => {
                if history.len() > 0 {
                    history_index = history_index.checked_sub(1).unwrap_or(0);
                    term.clear_line()?;
                    line = history[history_index].clone();
                    cursor_x = line.len();
                }
            }
            console::Key::ArrowDown => {
                if history.len() > 0 && history_index < history.len() - 1 {
                    history_index += 1;
                    term.clear_line()?;
                    line = history[history_index].clone();
                } else {
                    line.clear();
                    history_index = history.len();
                }
                cursor_x = line.len();
            }
            console::Key::Enter => {
                cursor_x = 0;
                term.clear_line()?;
                term.write(b"  ")?;
                write_colored_line(&mut term, &line)?;
                term.write_line("")?;
                let parse = if line.trim_end().ends_with(';') {
                    line.truncate(line.len() - 1);
                    term.write_line("")?;
                    true
                } else {
                    false
                };
                expr.push_str(&line);
                expr.push('\n');
                if parse {
                    let mut parser = Parser::new_with_top_level(&expr, None);
                    loop {
                        match parser.parse_error_details() {
                            Ok(exp) => {
                                let result = interpreter.interpret_stmt(exp.stmt);
                                match result {
                                    Ok(value) => term.write_fmt(format_args!("{value:?}"))?,
                                    Err(err) => term.write_fmt(format_args!("Error: {err}"))?,
                                };
                                let mut line = expr[exp.defined_at].to_string();
                                if line.starts_with('\n') {
                                    line.drain(0..1);
                                }
                                if line.ends_with('\n') {
                                    line.truncate(line.len() - 1);
                                }
                                for l in line.split('\n') {
                                    history.push(l.to_string());
                                }
                                history_index = history.len();
                                term.write_line("")?;
                            }
                            Err(ParserError {
                                info: ParserErrorInfo::Eof,
                                ..
                            }) => break,
                            Err(err) => {
                                term.write_line(&err.to_string())?;
                                term.write_line("")?;
                                break;
                            }
                        }
                    }
                    expr.clear();
                }
                line.clear();
            }
            console::Key::Backspace => {
                if !line.is_empty() {
                    line.drain(cursor_x.checked_sub(1).unwrap_or_default()..cursor_x);
                    cursor_x = cursor_x.checked_sub(1).unwrap_or(0);
                }
            }
            console::Key::Del => {
                if !line.is_empty() {
                    line.drain(cursor_x..(cursor_x + 1).min(line.len()));
                }
            }
            console::Key::Home => {
                cursor_x = 0;
            }
            console::Key::End => {
                cursor_x = line.len();
            }
            console::Key::Char(c) => {
                line.replace_range(cursor_x..cursor_x, &c.to_string());
                cursor_x += 1;
            }
            _ => {}
        }

        term.clear_line()?;
        term.write("> ".as_bytes())?;
        let term_width = term.size().1 as _;
        write_colored_line(
            &mut term,
            &line[cursor_x.checked_sub(term_width - 2).unwrap_or_default()..],
        )?;
        term.move_cursor_to((cursor_x + 2).min(term_width), term.size().0 as usize)?;
    }
}

fn write_colored_line(term: &mut Term, line: &str) -> Result<(), Error> {
    let mut scanner = Scanner::new(line);
    let mut final_string = String::with_capacity(132);
    let mut is_string = false;
    let mut is_interp = false;
    loop {
        let style = match scanner.advance() {
            Token::Eof => break,
            Token::LeftCurly if is_string => {
                is_interp = !is_interp;
                Style::new().bright().yellow()
            }
            Token::RightCurly if is_string => {
                is_interp = !is_interp;
                Style::new().bright().yellow()
            }
            Token::DoubleQuote => {
                is_string = !is_string;
                Style::new().bright().yellow()
            }
            _ if is_string && !is_interp => Style::new().bright().yellow(),
            Token::Number => Style::new().bright().blue(),
            t if t.is_keyword() => Style::new().red(),
            t if t != Token::Ident => Style::new().bright().red(),
            Token::Ident => Style::new().white(),
            _ => Style::new(),
        };
        final_string.push_str(&format!("{}", style.apply_to(scanner.slice())))
    }
    term.write(final_string.as_bytes())?;
    Ok(())
}
