# Toy Payment Engine

This is an implementation of a basic payments and transactions engine using some
concepts from a role based paradigm called [DCI](https://dci.github.io/).

You can run the program with `RUST_LOG=info` variable,
this would print out more log messages and information.
Otherwise, the program should print logs only upon errors.

### Notes
1. The getters for account and transaction data types copy the values when you read them.
   This is done to avoid return a reference that is state related, and can cause mistakes later.
   When you read a value, of `Account` the contract is it will not later change,
   and represent only the state in the time of read.
2. It is assumed the special operations
   like `Dispute`/`Resolve`/`Chargeback` can apply only for `Withdrawal`.

### Usage
```
cargo run -- transactions.csv > accounts.csv
```

Example `transactions.csv`
```
type,  client,     tx, amount
deposit,    1,      1,    1.0
deposit,    2,      2,    2.0
deposit,    1,      3,    2.0
withdrawal, 1,      4,    1.5
withdrawal, 2,      5,    3.0
deposit,    3,      6,    9.1234
withdrawal, 3,      7,    3.1234
dispute,    3,      7,
resolve,    3,      7,
deposit,    4,      8,    10.0
withdrawal, 4,      9,    1.95
dispute,    4,      9,
chargeback, 4,      9,
```
