use std::{error::Error, fmt::Display};

use scanner::Token;

#[derive(Debug)]
pub enum ParserErrorInfo {
    UnexpectedTokenExpected { found: Token, expected: Token },
    UnexpectedToken { found: Token },
    ExpectedIdentifierFoundKeyword { keyword: String },
    InvalidUseOfKeyword { keyword: String },
    UnableToParse { expr: String, err: String },
    EarlyEof,
    Eof,
    InvalidIndentation,
    InvalidLevelForItem,
    ExpectedExpresion,
    InvalidStructFieldType(String),
    InvalidEnumVariantType(String),
    InvalidTypeRef,
    DuplicateGenericName(String),
    InvalidExpression { found: String, expected: String },
    StatementNotValidInThisPosition,
    ExpressionNotValidInThisPosition,
    InvalidOperandForRangeExpression,
    InvalidCharacterLiteral,
}

impl Display for ParserErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            ParserErrorInfo::UnexpectedTokenExpected { found, expected } => {
                format!("Unexpected token: '{found:?}'. Expected: '{expected:?}'.")
            }
            ParserErrorInfo::UnexpectedToken { found } => {
                format!("Unexpected token '{found:?}'.")
            }
            ParserErrorInfo::EarlyEof => {
                format!("Early end of file.")
            }
            ParserErrorInfo::ExpectedIdentifierFoundKeyword { keyword } => {
                format!("Expected identifier, found: '{keyword}'.")
            }
            ParserErrorInfo::UnableToParse { expr, err } => {
                format!("Unable to parse '{expr}'. Error: {err}.")
            }
            ParserErrorInfo::Eof => format!("End of file."),
            ParserErrorInfo::InvalidIndentation => format!("Invalid indentation."),
            ParserErrorInfo::InvalidUseOfKeyword { keyword } => {
                format!("Invalid use of keyword '{keyword}'.")
            }
            ParserErrorInfo::InvalidLevelForItem => format!("Item cannot be a top level item."),
            ParserErrorInfo::ExpectedExpresion => format!("Expected at least one expression."),
            ParserErrorInfo::InvalidStructFieldType(name) => {
                format!("Invalid type for field with name: '{name}'.")
            }
            ParserErrorInfo::InvalidEnumVariantType(name) => {
                format!("Invalid type for variant with name: '{name}'.")
            }
            ParserErrorInfo::InvalidTypeRef => format!("Invalid type ref."),
            ParserErrorInfo::DuplicateGenericName(ident) => {
                format!("Duplicate generic argument name: '{ident}'.")
            }
            ParserErrorInfo::InvalidExpression { found, expected } => {
                format!("Invalid expression of type '{found}'. Expected: '<{expected}>'")
            }
            ParserErrorInfo::StatementNotValidInThisPosition => {
                format!("A statement is not valid in this position.")
            }
            ParserErrorInfo::ExpressionNotValidInThisPosition => {
                format!("An expression is not valid in this position.")
            }
            ParserErrorInfo::InvalidOperandForRangeExpression => {
                format!("This operand is invalid for a range expression.")
            }
            ParserErrorInfo::InvalidCharacterLiteral => {
                format!("Invalid character literal expression.")
            }
        };
        f.write_str(&fmt)
    }
}

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub location: String,
    pub info: ParserErrorInfo,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.info.fmt(f)?;
        f.write_str(format!("\n{}\nAt location {}", self.message, self.location).as_str())
    }
}

impl Error for ParserError {}
