use crate::{is_special_transaction, Account, BankContext, Transaction, TransactionType};
use std::collections::HashMap;

use crate::roles::{
    DepositAccount, HoldFundsAccount, LockedAccount, ReleaseFundsAccount, TransactionProcessor,
    WithdrawalAccount,
};
use crate::utility::{filter_real_transactions, is_withdrawal, round_amount};

#[test]
fn test_account() {
    let account = Account::new(1, 1_000.0);
    assert_eq!(account.identifier(), 1);
    assert_eq!(account.balance(), 1_000.0);
    assert_eq!(account.available(), 1_000.0);
    assert_eq!(account.held(), 0.0);
    assert_eq!(account.locked(), false);
}

#[test]
fn test_account_deposit() {
    let mut account = Account::new(1, 1_000.0);
    account.increment(500.0);
    assert_eq!(account.balance(), 1_500.0);
}

#[test]
fn test_account_withdrawal() {
    let mut account = Account::new(1, 1_000.0);
    account.decrement(750.0);
    assert_eq!(account.balance(), 250.0);
}

#[test]
fn test_account_hold_funds() {
    let mut account = Account::new(1, 1_000.0);
    account.hold_funds(100.0);
    assert_eq!(account.held(), 100.0);
    assert_eq!(account.available(), 900.0);
    assert_eq!(account.balance(), 1_000.0);
}

#[test]
fn test_account_release_funds() {
    let mut account = Account::new(1, 1_000.0);
    account.hold_funds(500.0);
    assert_eq!(account.held(), 500.0);

    account.release_funds(250.0);
    assert_eq!(account.available(), 750.0);
    assert_eq!(account.balance(), 1_000.0);
}

#[test]
fn test_account_lock() {
    let mut account = Account::new(1, 1_000.0);

    account.lock();
    assert_eq!(account.locked(), true);

    account.unlock();
    assert_eq!(account.locked(), false);
}

#[test]
fn test_bank_processing() {
    let account_id_1: u16 = 1;
    let account_id_2: u16 = 2;

    let mut accounts: HashMap<u16, Account> = HashMap::new();
    accounts.insert(account_id_1, Account::new(1, 1_000.0));
    let mut bank = BankContext::new(&mut accounts);

    let transaction1 = Transaction::new(1, 1, TransactionType::Withdrawal, Some(500.0));
    bank.process(&transaction1, None);

    let transaction2 = Transaction::new(1, 2, TransactionType::Deposit, Some(250.0));
    bank.process(&transaction2, None);

    let transaction3 = Transaction::new(2, 3, TransactionType::Deposit, Some(900.75));
    bank.process(&transaction3, None);

    let account1 = accounts.get(&account_id_1).unwrap();
    assert_eq!(account1.balance(), 750.0);
    assert_eq!(account1.available(), 750.0);

    let account2 = accounts.get(&account_id_2).unwrap();
    assert_eq!(account2.balance(), 900.75);
    assert_eq!(account2.available(), 900.75);
}

#[test]
fn test_is_special_transaction() {
    let transaction1 = Transaction::new(1, 1, TransactionType::Deposit, Some(1.0));
    let transaction2 = Transaction::new(1, 1, TransactionType::Withdrawal, Some(1.0));
    let transaction3 = Transaction::new(1, 1, TransactionType::Dispute, None);
    let transaction4 = Transaction::new(1, 1, TransactionType::Resolve, None);
    let transaction5 = Transaction::new(1, 1, TransactionType::Chargeback, None);

    assert_eq!(is_special_transaction(&transaction1), false);
    assert_eq!(is_special_transaction(&transaction2), false);
    assert_eq!(is_special_transaction(&transaction3), true);
    assert_eq!(is_special_transaction(&transaction4), true);
    assert_eq!(is_special_transaction(&transaction5), true);
}

#[test]
fn test_round_amount() {
    assert_eq!(round_amount(1.12344), 1.1234);
    assert_eq!(round_amount(1.12345), 1.1235);
    assert_eq!(round_amount(1.12346), 1.1235);
}

#[test]
fn test_is_withdrawal() {
    let transaction1 = Transaction::new(1, 1, TransactionType::Withdrawal, Some(1.0));
    let transaction2 = Transaction::new(1, 1, TransactionType::Deposit, Some(1.0));
    assert_eq!(is_withdrawal(&transaction1), true);
    assert_eq!(is_withdrawal(&transaction2), false);
}

#[test]
fn test_filter_real_transactions() {
    let transaction1 = Transaction::new(1, 1, TransactionType::Withdrawal, Some(1.0));
    let transaction2 = Transaction::new(1, 1, TransactionType::Deposit, Some(1.0));
    let transaction3 = Transaction::new(1, 1, TransactionType::Chargeback, None);

    let t1_ref = &transaction1;
    let t2_ref = &transaction2;
    let t3_ref = &transaction3;

    assert_eq!(filter_real_transactions(&t1_ref), true);
    assert_eq!(filter_real_transactions(&t2_ref), true);
    assert_eq!(filter_real_transactions(&t3_ref), false);
}
