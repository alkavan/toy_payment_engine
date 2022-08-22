# Toy Payment Engine

This is an implementation of a basic payments and transactions engine using some
concepts from a role based paradigm called [DCI](https://dci.github.io/).

You can run the program with `RUST_LOG=info` variable,
this would print out more log messages and information.
Otherwise, the program should print logs only upon errors.

### Notes
1. The getters for account and transaction datatypes copy the values when you read them.
   This is done to avoid return a reference that is state related, and can cause mistakes later.
   When you read a value, of `Account` the contract is it will not later change,
   and represent only the state in the team of read.
2. It is assumed the special operations
   like `Dispute`/`Resolve`/`Chargeback` can apply only for `Withdrawal`.
