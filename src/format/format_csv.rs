//! CSV format parser for YPBank records.

use crate::error::ParserError;
use crate::parser::Parser;
use crate::storage::{YPBankRecord, YPBankRecordStatus, YPBankRecordType, YPBankStorage};
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

const HEADER: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

/// Parser for the CSV record format.
pub struct CsvParser {
    /// In-memory storage populated after parsing.
    pub storage: YPBankStorage,
}

impl Parser for CsvParser {
    fn from_read<R: Read>(r: &mut R) -> Result<YPBankStorage, ParserError> {
        let mut storage = YPBankStorage::new();
        let mut reader = BufReader::new(r);
        parse_header(&mut reader)?;
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).map_err(io_error)?;
            if bytes_read == 0 {
                break; // EOF
            }
            if line.trim().is_empty() {
                continue; // skip empty lines
            }
            let record = parse_record(&line)?;
            storage.push(record);
        }
        Ok(storage)
    }

    fn write_to<W: Write>(&mut self, w: &mut W) -> Result<(), ParserError> {
        w.write_all(HEADER.as_bytes()).map_err(io_error)?;
        w.write_all(b"\n").map_err(io_error)?;
        for record in self.storage.records() {
            w.write_all(serialize_record(record).as_bytes())
                .map_err(io_error)?;
            w.write_all(b"\n").map_err(io_error)?;
        }
        Ok(())
    }

    fn from_storage(storage: YPBankStorage) -> Self {
        Self { storage }
    }
}

fn parse_header(r: &mut impl BufRead) -> Result<(), ParserError> {
    let mut header = String::new();
    r.read_line(&mut header).map_err(io_error)?;
    if header.trim() != HEADER {
        return Err(invalid_record("invalid CSV header"));
    }
    Ok(())
}

fn parse_record(line: &str) -> Result<YPBankRecord, ParserError> {
    let mut parts = line.splitn(8, ',');

    let tx_id = parts
        .next()
        .ok_or_else(|| invalid_record("missing TX_ID"))?
        .trim()
        .parse::<u64>()
        .map_err(|_| invalid_record("invalid TX_ID"))?;

    let tx_type = parse_tx_type(
        parts
            .next()
            .ok_or_else(|| invalid_record("missing TX_TYPE"))?
            .trim(),
    )?;

    let from_user_id = parts
        .next()
        .ok_or_else(|| invalid_record("missing FROM_USER_ID"))?
        .trim()
        .parse::<u64>()
        .map_err(|_| invalid_record("invalid FROM_USER_ID"))?;

    let to_user_id = parts
        .next()
        .ok_or_else(|| invalid_record("missing TO_USER_ID"))?
        .trim()
        .parse::<u64>()
        .map_err(|_| invalid_record("invalid TO_USER_ID"))?;

    let amount = parts
        .next()
        .ok_or_else(|| invalid_record("missing AMOUNT"))?
        .trim()
        .parse::<u64>()
        .map_err(|_| invalid_record("invalid AMOUNT"))?;

    let timestamp = parts
        .next()
        .ok_or_else(|| invalid_record("missing TIMESTAMP"))?
        .trim()
        .parse::<u64>()
        .map_err(|_| invalid_record("invalid TIMESTAMP"))?;

    let status = parse_status(
        parts
            .next()
            .ok_or_else(|| invalid_record("missing STATUS"))?
            .trim(),
    )?;

    let description_raw = parts
        .next()
        .ok_or_else(|| invalid_record("missing DESCRIPTION"))?
        .trim();
    let description = parse_description(description_raw)?;

    Ok(YPBankRecord {
        tx_id,
        tx_type,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        status,
        description,
    })
}

fn parse_tx_type(s: &str) -> Result<YPBankRecordType, ParserError> {
    YPBankRecordType::from_str(s).map_err(|_| invalid_record("invalid TX_TYPE"))
}

fn parse_status(s: &str) -> Result<YPBankRecordStatus, ParserError> {
    YPBankRecordStatus::from_str(s).map_err(|_| invalid_record("invalid STATUS"))
}

fn parse_description(s: &str) -> Result<String, ParserError> {
    if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
        Ok(s[1..s.len() - 1].to_string())
    } else {
        Err(invalid_record(
            "DESCRIPTION must be enclosed in double quotes",
        ))
    }
}

fn serialize_record(record: &YPBankRecord) -> String {
    format!(
        "{},{},{},{},{},{},{},\"{}\"",
        record.tx_id,
        record.tx_type,
        record.from_user_id,
        record.to_user_id,
        record.amount,
        record.timestamp,
        record.status,
        record.description
    )
}

fn invalid_record(msg: &str) -> ParserError {
    ParserError::InvalidRecord {
        message: msg.to_string(),
    }
}

fn io_error(e: std::io::Error) -> ParserError {
    ParserError::IO {
        message: e.to_string(),
        error: e,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn sample_record() -> YPBankRecord {
        YPBankRecord {
            tx_id: 43,
            tx_type: YPBankRecordType::TRANSFER,
            from_user_id: 1,
            to_user_id: 2,
            amount: 500,
            timestamp: 1700000000,
            status: YPBankRecordStatus::SUCCESS,
            description: "test transfer".to_string(),
        }
    }

    #[test]
    fn test_write_then_read() {
        let record = sample_record();
        let mut storage = YPBankStorage::new();
        storage.push(record.clone());

        let mut buf = Vec::new();
        let mut parser = CsvParser::from_storage(storage);
        parser.write_to(&mut buf).expect("write failed");

        let mut cursor = Cursor::new(buf);
        let parsed = CsvParser::from_read(&mut cursor).expect("read failed");

        assert_eq!(parsed.records().len(), 1);
        assert_eq!(parsed.records()[0], record);
    }

    #[test]
    fn test_read_from_csv() {
        let record = sample_record();
        let text = concat!(
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n",
            "43,TRANSFER,1,2,500,1700000000,SUCCESS,\"test transfer\"\n",
        );

        let mut cursor = Cursor::new(text);
        let parsed = CsvParser::from_read(&mut cursor).expect("read failed");

        assert_eq!(parsed.records().len(), 1);
        assert_eq!(parsed.records()[0], record);
    }
}
