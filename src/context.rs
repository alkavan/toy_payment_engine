extern crate pretty_env_logger;

use crate::data::{Account, TransactionType};
use crate::roles::{
    DepositAccount, HoldFundsAccount, LockedAccount, ReleaseFundsAccount, TransactionProcessor,
    WithdrawalAccount,
};
use crate::utility::{is_withdrawal, round_amount};
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

impl HoldFundsAccount for Account {
    fn hold_funds(&mut self, amount: f32) -> f32 {
        self.held_mut().add_assign(amount);
        return self.held();
    }
}

impl ReleaseFundsAccount for Account {
    fn release_funds(&mut self, amount: f32) -> f32 {
        self.held_mut().sub_assign(amount);
        return self.held();
    }
}

impl LockedAccount for Account {
    fn lock(&mut self) {
        *self.locked_mut() = true;
    }

    fn unlock(&mut self) {
        *self.locked_mut() = false;
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
    fn process(&mut self, transaction: &Transaction, parent: Option<&Transaction>) {
        let account_id = transaction.account_id();

        // if we don't have such account, create a new one, and add to mapping.
        if !self.accounts.contains_key(&account_id) {
            let account = Account::new(account_id.clone(), 0.0);
            self.accounts.insert(account.identifier(), account);
        }

        let account = self.accounts.get_mut(&account_id).unwrap();

        // it makes some sense that you cannot process transactions on locked accounts.
        // ar at least take different flow, but we will NOT handle it now.
        // if account.locked() {
        //     return;
        // }

        match transaction.transaction_type() {
            TransactionType::Unknown => {
                warn!(
                    "unknown transaction type (tx: {}), skipping.",
                    transaction.transaction_id()
                )
            }
            TransactionType::Deposit => {
                let amount = transaction.transaction_amount().unwrap();
                account.increment(round_amount(amount));
            }
            TransactionType::Withdrawal => {
                let amount = transaction.transaction_amount().unwrap();
                account.decrement(round_amount(amount));
            }
            TransactionType::Dispute => {
                if parent.is_none() {
                    // ignore if no parent transaction, this is probably an error
                    warn!(
                        "unable to handle dispute transaction (tx: {}), no parent found.",
                        transaction.transaction_id()
                    )
                } else {
                    let parent_transaction = parent.unwrap();
                    if is_withdrawal(&parent_transaction) {
                        // we assume this transaction has amount
                        let hold_amount =
                            round_amount(parent_transaction.transaction_amount().unwrap());
                        account.hold_funds(hold_amount);
                    }
                }
            }
            TransactionType::Resolve => {
                if parent.is_none() {
                    // ignore if no parent transaction, this is probably an error
                    warn!(
                        "unable to handle dispute transaction (tx: {}), no parent found.",
                        transaction.transaction_id()
                    )
                } else {
                    // it is possible we got a resolve transaction, but didn't get the dispute.
                    // we don't handle this scenario, or keep track of it.
                    let parent_transaction = parent.unwrap();
                    if is_withdrawal(&parent_transaction) {
                        let hold_amount =
                            round_amount(parent_transaction.transaction_amount().unwrap());
                        account.release_funds(hold_amount);
                        // we add the fund back to the account, reversing the transaction
                        account.increment(hold_amount);
                    }
                }
            }
            TransactionType::Chargeback => {
                if parent.is_none() {
                    // ignore if no parent transaction, this is probably an error
                    warn!(
                        "unable to handle dispute transaction (tx: {}), no parent found.",
                        transaction.transaction_id()
                    )
                } else {
                    let parent_transaction = parent.unwrap();
                    if is_withdrawal(&parent_transaction) {
                        let hold_amount = parent.unwrap().transaction_amount().unwrap();
                        account.release_funds(round_amount(hold_amount));
                        // the withdrawal already made, so we don't change the actual funds.
                        account.lock();
                    }
                }
            }
        }
    }
}
