//! YPBank transaction log parser library.
//!
//! Supports reading and writing bank records in TXT, CSV, and binary formats.
#![warn(missing_docs)]
pub mod cli;
pub mod error;
pub mod format;
pub mod parser;
pub mod storage;
