//! Error types used throughout the parser library

use std::io::Error as IoError;
use thiserror::Error;

/// Error types for CLI operations
#[derive(Error, Debug)]
pub enum CliError {
    /// An unrecognized argument was provided
    #[error("unknown argument: {name}")]
    UnknownArgument {
        /// The name of the unknown argument
        name: String,
    },

    /// A required argument was not provided
    #[error("missing required argument: {name}")]
    MissingArgument {
        /// The name of the missing argument
        name: String,
    },

    /// An argument was provided without its expected value
    #[error("missing value for argument: {name}")]
    MissingValue {
        /// The name of the argument missing a value
        name: String,
    },

    /// An unsupported or unknown format was specified
    #[error("invalid format: {name}")]
    InvalidFormat {
        /// The invalid format string
        name: String,
    },

    /// An I/O error occurred during CLI processing
    #[error("I/O error: {message}")]
    IO {
        /// Human-readable error description
        message: String,
        /// The underlying I/O error
        error: IoError,
    },

    /// A parser error propagated from the parsing stage
    #[error("parser error")]
    Parser(#[from] ParserError),
}

/// Error types for parser operations
#[derive(Error, Debug)]
pub enum ParserError {
    /// The record data is malformed or missing required fields
    #[error("invalid record: {message}")]
    InvalidRecord {
        /// Human-readable description of what is invalid
        message: String,
    },

    /// An I/O error occurred while reading or writing record data
    #[error("I/O error: {message}")]
    IO {
        /// Human-readable error description
        message: String,
        /// The underlying I/O error
        error: IoError,
    },
}
