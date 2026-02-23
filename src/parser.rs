//! Core parser trait definition.

use crate::error::ParserError;
use crate::storage::YPBankStorage;

/// Trait for parsing and writing YPBankStorage data
pub trait Parser {
    /// Reads data from reader
    fn from_read<R: std::io::Read>(r: &mut R) -> Result<YPBankStorage, ParserError>;

    /// Writes data to writer
    fn write_to<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), ParserError>;

    /// Creates new parser from storage
    fn from_storage(storage: YPBankStorage) -> Self;
}
