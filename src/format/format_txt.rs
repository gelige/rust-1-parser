use crate::error::ParserError;
use crate::parser::Parser;
use crate::storage::{YPBankRecord, YPBankRecordStatus, YPBankRecordType, YPBankStorage};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

pub struct TxtParser {
    pub storage: YPBankStorage,
}

impl Parser for TxtParser {
    fn from_read<R: Read>(r: &mut R) -> Result<YPBankStorage, ParserError> {
        let mut storage = YPBankStorage::new();
        let reader = BufReader::new(r);
        let mut fields: HashMap<String, String> = HashMap::new();

        for line_result in reader.lines() {
            let line = line_result.map_err(io_error)?.trim().to_string();

            if line.starts_with('#') {
                continue;
            }

            if line.is_empty() {
                if !fields.is_empty() {
                    storage.push(build_record(&mut fields)?);
                }
                continue;
            }

            let (key, value) = parse_key_value(&line)?;
            if fields.contains_key(key) {
                return Err(invalid_record(&format!("duplicate field: {}", key)));
            }
            fields.insert(key.to_string(), value.to_string());
        }

        if !fields.is_empty() {
            storage.push(build_record(&mut fields)?);
        }

        Ok(storage)
    }

    fn write_to<W: Write>(&mut self, w: &mut W) -> Result<(), ParserError> {
        let records = self.storage.records();
        for (i, record) in records.iter().enumerate() {
            w.write_all(serialize_record(record).as_bytes())
                .map_err(io_error)?;
            if i + 1 < records.len() {
                w.write_all(b"\n").map_err(io_error)?;
            }
        }
        Ok(())
    }

    fn from_storage(storage: YPBankStorage) -> Self {
        Self { storage }
    }
}

fn parse_key_value(line: &str) -> Result<(&str, &str), ParserError> {
    let pos = line
        .find(": ")
        .ok_or_else(|| invalid_record("expected 'KEY: VALUE' format"))?;
    Ok((&line[..pos], &line[pos + 2..]))
}

fn build_record(fields: &mut HashMap<String, String>) -> Result<YPBankRecord, ParserError> {
    let tx_id = take_field(fields, "TX_ID")?
        .parse::<u64>()
        .map_err(|_| invalid_record("invalid TX_ID"))?;

    let tx_type = parse_tx_type(&take_field(fields, "TX_TYPE")?)?;

    let from_user_id = take_field(fields, "FROM_USER_ID")?
        .parse::<u64>()
        .map_err(|_| invalid_record("invalid FROM_USER_ID"))?;

    let to_user_id = take_field(fields, "TO_USER_ID")?
        .parse::<u64>()
        .map_err(|_| invalid_record("invalid TO_USER_ID"))?;

    let amount = take_field(fields, "AMOUNT")?
        .parse::<u64>()
        .map_err(|_| invalid_record("invalid AMOUNT"))?;

    let timestamp = take_field(fields, "TIMESTAMP")?
        .parse::<u64>()
        .map_err(|_| invalid_record("invalid TIMESTAMP"))?;

    let status = parse_status(&take_field(fields, "STATUS")?)?;

    let description_raw = take_field(fields, "DESCRIPTION")?;
    let description = parse_description(&description_raw)?;

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

fn take_field(fields: &mut HashMap<String, String>, key: &str) -> Result<String, ParserError> {
    fields
        .remove(key)
        .ok_or_else(|| invalid_record(&format!("missing field: {}", key)))
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
        "TX_ID: {}\nTX_TYPE: {}\nFROM_USER_ID: {}\nTO_USER_ID: {}\nAMOUNT: {}\nTIMESTAMP: {}\nSTATUS: {}\nDESCRIPTION: \"{}\"\n",
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn sample_record() -> YPBankRecord {
        YPBankRecord {
            tx_id: 44,
            tx_type: YPBankRecordType::WITHDRAWAL,
            from_user_id: 1,
            to_user_id: 2,
            amount: 500,
            timestamp: 1700000000,
            status: YPBankRecordStatus::FAILURE,
            description: "test withdrawal".to_string(),
        }
    }

    #[test]
    fn test_write_then_read() {
        let record = sample_record();
        let mut storage = YPBankStorage::new();
        storage.push(record.clone());

        let mut buf = Vec::new();
        let mut parser = TxtParser::from_storage(storage);
        parser.write_to(&mut buf).expect("write failed");

        let mut cursor = Cursor::new(buf);
        let parsed = TxtParser::from_read(&mut cursor).expect("read failed");

        assert_eq!(parsed.records().len(), 1);
        assert_eq!(parsed.records()[0], record);
    }

    #[test]
    fn test_read_from_text() {
        let record = sample_record();
        let text = concat!(
            "TX_ID: 44\n",
            "TX_TYPE: WITHDRAWAL\n",
            "FROM_USER_ID: 1\n",
            "TO_USER_ID: 2\n",
            "AMOUNT: 500\n",
            "TIMESTAMP: 1700000000\n",
            "STATUS: FAILURE\n",
            "DESCRIPTION: \"test withdrawal\"\n",
        );

        let mut cursor = Cursor::new(text);
        let parsed = TxtParser::from_read(&mut cursor).expect("read failed");

        assert_eq!(parsed.records().len(), 1);
        assert_eq!(parsed.records()[0], record);
    }
}
