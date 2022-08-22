use crate::Transaction;

pub trait DepositAccount {
    fn increment(&mut self, amount: f32) -> f32;
}

pub trait WithdrawalAccount {
    fn decrement(&mut self, amount: f32) -> f32;
}

pub trait HoldFundsAccount {
    fn hold_funds(&mut self, amount: f32) -> f32;
}

pub trait ReleaseFundsAccount {
    fn release_funds(&mut self, amount: f32) -> f32;
}

pub trait LockedAccount {
    fn lock(&mut self);
    fn unlock(&mut self);
}

pub trait TransactionProcessor {
    fn process(&mut self, transaction: &Transaction, parent: Option<&Transaction>);
}
