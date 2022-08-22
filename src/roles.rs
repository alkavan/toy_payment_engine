use crate::Transaction;

pub trait DepositAccount {
    fn increment(&mut self, amount: f32) -> f32;
}

pub trait WithdrawalAccount {
    fn decrement(&mut self, amount: f32) -> f32;
}

pub trait TransactionProcessor {
    fn process(&mut self, transaction: Transaction);
}
