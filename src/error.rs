use std::fmt;

/// Error types for CLI operations
#[derive(Debug)]
pub enum CliError {
    UnknownArgument { name: String },
    MissingArgument { name: String },
    MissingValue { name: String },
    InvalidFormat { name: String },
    IO { message: String },
    Parser { message: String },
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::UnknownArgument { name } => write!(f, "unknown argument: {}", name),
            CliError::MissingArgument { name } => write!(f, "missing required argument: {}", name),
            CliError::MissingValue { name } => write!(f, "missing value for argument: {}", name),
            CliError::InvalidFormat { name } => write!(f, "invalid format: {}", name),
            CliError::IO { message } => write!(f, "I/O error: {}", message),
            CliError::Parser { message } => write!(f, "parser error: {}", message),
        }
    }
}

impl std::error::Error for CliError {}

/// Error types for parser operations
#[derive(Debug)]
pub enum ParserError {
    InvalidRecord { message: String },
    IO { message: String },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidRecord { message } => write!(f, "invalid record: {}", message),
            ParserError::IO { message } => write!(f, "I/O error: {}", message),
        }
    }
}

impl std::error::Error for ParserError {}

impl From<ParserError> for CliError {
    fn from(e: ParserError) -> Self {
        CliError::Parser {
            message: e.to_string(),
        }
    }
}
