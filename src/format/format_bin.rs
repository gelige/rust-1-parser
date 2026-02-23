use crate::error::ParserError;
use crate::parser::Parser;
use crate::storage::{YPBankRecord, YPBankRecordStatus, YPBankRecordType, YPBankStorage};
use std::io::{Cursor, Read, Write};

const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E]; // 'YPBN'

pub struct BinParser {
    pub storage: YPBankStorage,
}

impl Parser for BinParser {
    fn from_read<R: Read>(r: &mut R) -> Result<YPBankStorage, ParserError> {
        let mut storage = YPBankStorage::new();
        loop {
            // Read record header
            let mut magic = [0u8; 4];
            if r.read_exact(&mut magic).is_err() {
                break; // EOF
            }
            if magic != MAGIC {
                return Err(invalid_record("invalid record header"));
            }

            // Record size
            let record_size = read_u32_be(r)? as usize;
            let mut body = vec![0u8; record_size];
            r.read_exact(&mut body)
                .map_err(|_| invalid_record("invalid record body"))?;
            storage.push(parse_record_body(&body)?);
        }
        Ok(storage)
    }

    fn write_to<W: Write>(&mut self, w: &mut W) -> Result<(), ParserError> {
        for record in self.storage.records() {
            let body = serialize_record(record);
            w.write_all(&MAGIC).map_err(io_error)?;
            w.write_all(&(body.len() as u32).to_be_bytes())
                .map_err(io_error)?;
            w.write_all(&body).map_err(io_error)?;
        }
        Ok(())
    }

    fn from_storage(storage: YPBankStorage) -> Self {
        Self { storage }
    }
}

fn parse_record_body(body: &[u8]) -> Result<YPBankRecord, ParserError> {
    let mut cur = Cursor::new(body);
    let tx_id = read_u64_be(&mut cur)?;
    let tx_type = read_tx_type(&mut cur)?;
    let from_user_id = read_u64_be(&mut cur)?;
    let to_user_id = read_u64_be(&mut cur)?;
    let amount = read_i64_be(&mut cur)?.unsigned_abs();
    let timestamp = read_u64_be(&mut cur)?;
    let status = read_status(&mut cur)?;
    let desc_len = read_u32_be(&mut cur)? as usize;

    let mut desc_bytes = vec![0u8; desc_len];
    cur.read_exact(&mut desc_bytes)
        .map_err(|_| invalid_record("DESCRIPTION length exceeds body"))?;

    let description = String::from_utf8(desc_bytes)
        .map_err(|_| invalid_record("DESCRIPTION is not valid UTF-8"))?;

    let description = description.trim_matches('"').to_string();

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

fn serialize_record(record: &YPBankRecord) -> Vec<u8> {
    let desc = record.description.as_bytes();
    let mut out = Vec::with_capacity(46 + desc.len());
    out.extend_from_slice(&record.tx_id.to_be_bytes());
    out.push(match record.tx_type {
        YPBankRecordType::DEPOSIT => 0,
        YPBankRecordType::TRANSFER => 1,
        YPBankRecordType::WITHDRAWAL => 2,
    });
    out.extend_from_slice(&record.from_user_id.to_be_bytes());
    out.extend_from_slice(&record.to_user_id.to_be_bytes());
    let amount_i64: i64 = match record.tx_type {
        YPBankRecordType::WITHDRAWAL => -(record.amount as i64),
        _ => record.amount as i64,
    };
    out.extend_from_slice(&amount_i64.to_be_bytes());
    out.extend_from_slice(&record.timestamp.to_be_bytes());
    out.push(match record.status {
        YPBankRecordStatus::SUCCESS => 0,
        YPBankRecordStatus::FAILURE => 1,
        YPBankRecordStatus::PENDING => 2,
    });
    out.extend_from_slice(&(desc.len() as u32).to_be_bytes());
    out.extend_from_slice(desc);
    out
}

fn read_u32_be(r: &mut impl Read) -> Result<u32, ParserError> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)
        .map_err(|_| invalid_record("truncated field"))?;
    Ok(u32::from_be_bytes(b))
}

fn read_u64_be(r: &mut impl Read) -> Result<u64, ParserError> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)
        .map_err(|_| invalid_record("truncated field"))?;
    Ok(u64::from_be_bytes(b))
}

fn read_i64_be(r: &mut impl Read) -> Result<i64, ParserError> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)
        .map_err(|_| invalid_record("truncated field"))?;
    Ok(i64::from_be_bytes(b))
}

fn read_tx_type(r: &mut impl Read) -> Result<YPBankRecordType, ParserError> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b)
        .map_err(|_| invalid_record("truncated TX_TYPE"))?;
    match b[0] {
        0 => Ok(YPBankRecordType::DEPOSIT),
        1 => Ok(YPBankRecordType::TRANSFER),
        2 => Ok(YPBankRecordType::WITHDRAWAL),
        _ => Err(invalid_record("invalid TX_TYPE")),
    }
}

fn read_status(r: &mut impl Read) -> Result<YPBankRecordStatus, ParserError> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b)
        .map_err(|_| invalid_record("truncated STATUS"))?;
    match b[0] {
        0 => Ok(YPBankRecordStatus::SUCCESS),
        1 => Ok(YPBankRecordStatus::FAILURE),
        2 => Ok(YPBankRecordStatus::PENDING),
        _ => Err(invalid_record("invalid STATUS")),
    }
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
    use crate::storage::{YPBankRecord, YPBankRecordStatus, YPBankRecordType};

    fn sample_record() -> YPBankRecord {
        YPBankRecord {
            tx_id: 42,
            tx_type: YPBankRecordType::DEPOSIT,
            from_user_id: 1,
            to_user_id: 2,
            amount: 1000,
            timestamp: 1700000001,
            status: YPBankRecordStatus::PENDING,
            description: "test deposit".to_string(),
        }
    }

    #[test]
    fn test_write_then_read() {
        let record = sample_record();
        let mut storage = YPBankStorage::new();
        storage.push(record.clone());

        let mut buf = Vec::new();
        let mut parser = BinParser::from_storage(storage);
        parser.write_to(&mut buf).expect("write failed");

        let mut cursor = std::io::Cursor::new(buf);
        let parsed = BinParser::from_read(&mut cursor).expect("read failed");

        assert_eq!(parsed.records().len(), 1);
        assert_eq!(parsed.records()[0], record);
    }

    #[test]
    fn test_read_from_binary() {
        let record = sample_record();

        // Manually build the binary representation
        let body = serialize_record(&record);
        let mut data = Vec::new();
        data.extend_from_slice(&MAGIC);
        data.extend_from_slice(&(body.len() as u32).to_be_bytes());
        data.extend_from_slice(&body);

        let mut cursor = std::io::Cursor::new(data);
        let parsed = BinParser::from_read(&mut cursor).expect("read failed");

        assert_eq!(parsed.records().len(), 1);
        assert_eq!(parsed.records()[0], record);
    }
}
