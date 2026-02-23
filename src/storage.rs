use strum_macros::Display;
use strum_macros::EnumString;

/// Storage for YPBank records
pub struct YPBankStorage {
    records: Vec<YPBankRecord>,
}

impl YPBankStorage {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// Get all records
    pub fn records(&self) -> &[YPBankRecord] {
        &self.records
    }

    /// Get a record by index
    pub fn get(&self, index: usize) -> Option<&YPBankRecord> {
        self.records.get(index)
    }

    /// Push a new record to the storage
    pub fn push(&mut self, record: YPBankRecord) {
        self.records.push(record);
    }
}

/// A record in the YPBank storage
#[derive(Debug, PartialEq, Clone)]
pub struct YPBankRecord {
    pub tx_id: u64,
    pub tx_type: YPBankRecordType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub status: YPBankRecordStatus,
    pub description: Description,
}

pub type Description = String;

#[derive(Debug, PartialEq, Clone, Display, EnumString)]
pub enum YPBankRecordType {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
}

#[derive(Debug, PartialEq, Clone, Display, EnumString)]
pub enum YPBankRecordStatus {
    SUCCESS,
    FAILURE,
    PENDING,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_storage_is_empty() {
        let storage = YPBankStorage::new();
        assert!(storage.records().is_empty());
    }

    #[test]
    fn test_push_record() {
        let mut storage = YPBankStorage::new();
        let record = YPBankRecord {
            tx_id: 1,
            tx_type: YPBankRecordType::DEPOSIT,
            from_user_id: 1,
            to_user_id: 2,
            amount: 100,
            timestamp: 1638224000,
            status: YPBankRecordStatus::SUCCESS,
            description: "Some deposit".to_string(),
        };
        let expected = record.clone();
        storage.push(record);
        assert_eq!(storage.records().len(), 1);
        assert_eq!(storage.records(), &[expected]);
    }

    #[test]
    fn test_get_record() {
        let mut storage = YPBankStorage::new();
        let record = YPBankRecord {
            tx_id: 2,
            tx_type: YPBankRecordType::TRANSFER,
            from_user_id: 2,
            to_user_id: 3,
            amount: 120,
            timestamp: 1638224111,
            status: YPBankRecordStatus::PENDING,
            description: "Some pending transfer".to_string(),
        };
        let expected = record.clone();
        storage.push(record);
        assert_eq!(storage.get(0), Some(&expected));
        assert_eq!(storage.get(1), None);
    }
}
