use std::io::Error as IoError;
use thiserror::Error;

/// Error types for CLI operations
#[derive(Error, Debug)]
pub enum CliError {
    #[error("unknown argument: {name}")]
    UnknownArgument { name: String },

    #[error("missing required argument: {name}")]
    MissingArgument { name: String },

    #[error("missing value for argument: {name}")]
    MissingValue { name: String },

    #[error("invalid format: {name}")]
    InvalidFormat { name: String },

    #[error("I/O error: {message}")]
    IO { message: String, error: IoError },

    #[error("parser error")]
    Parser(#[from] ParserError),
}

/// Error types for parser operations
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("invalid record: {message}")]
    InvalidRecord { message: String },

    #[error("I/O error: {message}")]
    IO { message: String, error: IoError },
}
