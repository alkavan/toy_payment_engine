/**
* Account data structure
*/
pub struct Account {
    account_id: u16,
    balance: f32,
    held_funds: f32,
    locked: bool,
}

impl Account {
    pub fn new(account_id: u16, balance: f32) -> Account {
        return Account {
            account_id,
            balance,
            held_funds: 0.0,
            locked: false,
        };
    }

    pub fn identifier(&self) -> u16 {
        return self.account_id.clone();
    }

    pub fn balance(&self) -> f32 {
        return self.balance.clone();
    }

    pub fn balance_mut(&mut self) -> &mut f32 {
        return &mut self.balance;
    }

    pub fn available(&self) -> f32 {
        return self.balance - self.held_funds;
    }

    pub fn held(&self) -> f32 {
        return self.held_funds.clone();
    }

    pub fn held_mut(&mut self) -> &mut f32 {
        return &mut self.held_funds;
    }

    pub fn locked(&self) -> bool {
        return self.locked.clone();
    }

    pub fn locked_mut(&mut self) -> &mut bool {
        return &mut self.locked;
    }
}

/**
 * Types of transaction
 */
pub enum TransactionType {
    Unknown,
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl From<&str> for TransactionType {
    fn from(string: &str) -> Self {
        match string {
            "deposit" => TransactionType::Deposit,
            "withdrawal" => TransactionType::Withdrawal,
            "dispute" => TransactionType::Dispute,
            "resolve" => TransactionType::Resolve,
            "chargeback" => TransactionType::Chargeback,
            &_ => TransactionType::Unknown,
        }
    }
}

/**
 * Transaction data structure
 */
pub struct Transaction {
    transaction_id: u32,
    transaction_type: TransactionType,
    transaction_amount: Option<f32>,
    account_id: u16,
}

impl Transaction {
    pub fn new(
        account_id: u16,
        transaction_id: u32,
        transaction_type: TransactionType,
        transaction_amount: Option<f32>,
    ) -> Transaction {
        return Transaction {
            transaction_id,
            transaction_type,
            transaction_amount,
            account_id,
        };
    }

    pub fn transaction_id(&self) -> u32 {
        return self.transaction_id.clone();
    }

    pub fn transaction_type(&self) -> &TransactionType {
        return &self.transaction_type;
    }

    pub fn transaction_amount(&self) -> Option<f32> {
        return self.transaction_amount.clone();
    }

    pub fn account_id(&self) -> u16 {
        return self.account_id.clone();
    }
}
