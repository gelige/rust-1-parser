//! In-memory storage and data types for YPBank transaction records

use strum_macros::Display;
use strum_macros::EnumString;

/// Storage for YPBank records
pub struct YPBankStorage {
    records: Vec<YPBankRecord>,
}

impl YPBankStorage {
    /// Creates a new empty storage
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
    /// Unique transaction identifier
    pub tx_id: u64,
    /// Type of the transaction
    pub tx_type: YPBankRecordType,
    /// ID of the user sending the funds
    pub from_user_id: u64,
    /// ID of the user receiving the funds
    pub to_user_id: u64,
    /// Transaction amount in the smallest currency unit
    pub amount: u64,
    /// Unix timestamp of the transaction
    pub timestamp: u64,
    /// Current status of the transaction
    pub status: YPBankRecordStatus,
    /// Free-text description of the transaction
    pub description: Description,
}

/// A description attached to a transaction record
pub type Description = String;

/// Possible transaction types for a bank record
#[derive(Debug, PartialEq, Clone, Display, EnumString)]
pub enum YPBankRecordType {
    /// Funds added to an account
    DEPOSIT,
    /// Funds moved between two accounts
    TRANSFER,
    /// Funds removed from an account
    WITHDRAWAL,
}

/// Possible processing statuses for a bank record
#[derive(Debug, PartialEq, Clone, Display, EnumString)]
pub enum YPBankRecordStatus {
    /// Transaction completed successfully
    SUCCESS,
    /// Transaction failed
    FAILURE,
    /// Transaction is still being processed
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
