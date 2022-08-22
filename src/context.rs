extern crate pretty_env_logger;

use crate::data::{Account, TransactionType};
use crate::roles::{DepositAccount, TransactionProcessor, WithdrawalAccount};
use crate::Transaction;
use csv::StringRecord;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::ops::{AddAssign, SubAssign};

/**
 * Implementing deposit to account
 */
impl DepositAccount for Account {
    fn increment(&mut self, amount: f32) -> f32 {
        self.balance_mut().add_assign(amount);
        return self.balance();
    }
}

/**
 * Implementing withdrawal from account
 */
impl WithdrawalAccount for Account {
    fn decrement(&mut self, amount: f32) -> f32 {
        self.balance_mut().sub_assign(amount);
        return self.balance();
    }
}

/**
 * Implementing conversion of StringRecord from CSV file into Transaction
 */
impl From<StringRecord> for Transaction {
    fn from(record: StringRecord) -> Self {
        let transaction_type = record.get(0).unwrap().trim();
        let account_id: u16 = record.get(1).unwrap().trim().parse().unwrap();
        let transaction_id: u32 = record.get(2).unwrap().trim().parse().unwrap();

        let transaction_amount_string = record.get(3).unwrap().trim();
        let transaction_amount: Option<f32> = match transaction_amount_string.len() {
            0 => None,
            _ => Some(transaction_amount_string.parse().unwrap()),
        };

        Transaction::new(
            account_id,
            transaction_id,
            TransactionType::from(transaction_type),
            transaction_amount,
        )
    }
}

/**
 * CSV Import
 */
pub struct CsvImportContext<'a> {
    filepath: &'a str,
}

impl CsvImportContext<'_> {
    pub fn new(filepath: &str) -> CsvImportContext {
        CsvImportContext { filepath }
    }

    pub fn read(&self) -> Result<Vec<Transaction>, Box<dyn Error>> {
        let reader = csv::Reader::from_path(self.filepath);

        if let Err(err) = reader {
            error!("{}", err);
            return Err(format!("failed loading csv file: {}", self.filepath).into());
        }

        let transactions = reader?
            .records()
            .map(|record| Transaction::from(record.unwrap()))
            .collect();

        Ok(transactions)
    }
}

/**
 * CSV Export
 */
pub struct CsvExportContext<'a> {
    header: Vec<&'a str>,
}

impl CsvExportContext<'_> {
    pub fn new<'a>(header: Vec<&str>) -> CsvExportContext {
        CsvExportContext { header }
    }

    pub fn write<'a>(&self, accounts: &HashMap<u16, Account>) -> Result<(), Box<dyn Error>> {
        let mut writer = csv::Writer::from_writer(io::stdout());

        // write csv header
        writer.write_record(&self.header)?;

        for (_, account) in accounts.iter() {
            writer.write_record(&[
                account.identifier().to_string(),
                format!("{:.4}", account.available()),
                format!("{:.4}", account.held()),
                format!("{:.4}", account.balance()),
                format!("{}", account.locked()),
            ])?;
        }

        writer.flush()?;

        Ok(())
    }
}

/**
 * Bank
 */
pub struct BankContext<'a> {
    accounts: &'a mut HashMap<u16, Account>,
}

impl BankContext<'_> {
    pub fn new(accounts: &mut HashMap<u16, Account>) -> BankContext {
        BankContext { accounts }
    }
}

impl TransactionProcessor for BankContext<'_> {
    fn process(&mut self, transaction: Transaction) {
        let account_id = &transaction.account_id();
        if !self.accounts.contains_key(account_id) {
            let account = Account::new(account_id.clone(), 0.0);
            self.accounts.insert(account.identifier(), account);
        }

        let account = self.accounts.get_mut(account_id).unwrap();

        match transaction.transaction_type() {
            TransactionType::Unknown => {
                warn!(
                    "unknown transaction type (tx: {}), skipping.",
                    transaction.transaction_id()
                )
            }
            TransactionType::Deposit => {
                account.increment(transaction.transaction_amount().unwrap());
            }
            TransactionType::Withdrawal => {
                account.increment(transaction.transaction_amount().unwrap());
            }
            TransactionType::Dispute => {
                warn!("Dispute transaction type not implemented")
            }
            TransactionType::Resolve => {
                warn!("Resolve transaction type not implemented")
            }
            TransactionType::Chargeback => {
                warn!("Chargeback transaction type not implemented")
            }
        }
    }
}
