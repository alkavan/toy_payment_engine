extern crate round;

use crate::{Transaction, TransactionType};
use round::round;
use std::slice::Iter;

pub fn round_amount(amount: f32) -> f32 {
    round(amount as f64, 4) as f32
}

pub fn filter_real_transactions(transaction: &&Transaction) -> bool {
    matches!(transaction.transaction_type(), TransactionType::Deposit)
        || matches!(transaction.transaction_type(), TransactionType::Withdrawal)
}

/**
 * Filter only deposit and withdrawal transactions, and find one matches the identifier
 */
pub fn find_transaction_parent<'a>(
    transaction: &'a Transaction,
    transactions: Iter<'a, Transaction>,
) -> Option<&'a Transaction> {
    transactions
        .filter(filter_real_transactions)
        .find(|&t| t.transaction_id() == transaction.transaction_id())
}

pub fn is_special_transaction(transaction: &Transaction) -> bool {
    match transaction.transaction_type() {
        TransactionType::Unknown => false,
        TransactionType::Deposit => false,
        TransactionType::Withdrawal => false,
        TransactionType::Dispute => true,
        TransactionType::Resolve => true,
        TransactionType::Chargeback => true,
    }
}

pub fn is_withdrawal(transaction: &Transaction) -> bool {
    return matches!(transaction.transaction_type(), TransactionType::Withdrawal);
}
