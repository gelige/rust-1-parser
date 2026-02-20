use std::fmt::Display;
use std::fmt::Formatter;

pub struct YPBankStorage {
    records: Vec<YPBankRecord>,
}

impl YPBankStorage {
    pub fn get(&self, p0: i32) -> Option<&YPBankRecord> {
        self.records.get(p0 as usize)
    }
}

impl YPBankStorage {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn push(&mut self, record: YPBankRecord) {
        self.records.push(record);
    }

    pub fn records(&self) -> &[YPBankRecord] {
        &self.records
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum YPBankRecordType {
    Deposit,
    Transfer,
    Withdrawal,
}

impl Display for YPBankRecordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                YPBankRecordType::Deposit => "DEPOSIT",
                YPBankRecordType::Transfer => "TRANSFER",
                YPBankRecordType::Withdrawal => "WITHDRAWAL",
            }
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum YPBankRecordStatus {
    Success,
    Failure,
    Pending,
}

impl Display for YPBankRecordStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                YPBankRecordStatus::Success => "SUCCESS",
                YPBankRecordStatus::Failure => "FAILURE",
                YPBankRecordStatus::Pending => "PENDING",
            }
        )?;
        Ok(())
    }
}
