use std::{env::args, error::Error, fmt::Display};

use parser::{Parser, ParserError, ParserErrorInfo};

#[derive(Debug)]
pub enum RunError {
    NoFileProvided,
    ParserError(ParserError),
}

impl Error for RunError {}

impl Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::NoFileProvided => f.write_str("Please provide a file to run."),
            RunError::ParserError(e) => e.fmt(f),
        }
    }
}

fn main() -> Result<(), RunError> {
    let mut args = args();

    if args.len() == 1 {
        return Err(RunError::NoFileProvided);
    }

    let filename = args.nth(1).unwrap();

    let file = std::fs::read_to_string(filename.clone()).expect("Unable to read file.");
    let mut parser = Parser::new(&file, Some(filename));

    loop {
        match parser.parse_error_details() {
            Err(ParserError {
                info: ParserErrorInfo::Eof,
                ..
            }) => break,
            Err(err) => {
                println!("{err}");
                return Err(RunError::ParserError(err));
            }
            Ok(stmt) => println!("{stmt:?}"),
        }
    }

    Ok(())
}
