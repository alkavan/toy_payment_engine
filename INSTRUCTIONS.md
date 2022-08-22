# Task Definition

## Objectives
The engine should a read series of transactions from a CSV file,
process the transactions, and output the state of clients accounts as a CSV.

### Features
* Handles disputes.
* Handles chargebacks.

### Notes
* Try to stream values through memory as opposed to loading the entire data set upfront.
* What if your code was bundled in a server,
and these CSVs came from thousands of concurrent TCP streams?
* Expose the correct CLI interface for automatic tests.

### CLI Interface
```
cargo run -- transactions.csv > accounts.csv
```
The input file is the first and only argument to the binary. Output should be written to `stdout`.

### Input
* **type** is `&str`
* **client** is `u16`
* **tx** is `u32`
* **amount** `f32`/`f64` (decide)
```
type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 3.0
```

* **client** field is globally unique, but order is not guaranteed.
* **tx** field is globally unique, but order is not guaranteed.
* Transactions occur chronologically in the file.
* Whitespaces and decimal precisions (up to four places past the decimal)
must be accepted by your program.

### Output

The output should be a list of:
* Client IDs (`client`)
* Available amounts (`available`)
* Held amounts (`held`)
* Total amounts (`total`)
* Whether the account is locked (`locked`)

```
client, available, held, total, locked
     1,       1.5,  0.0,   1.5,  false
     2,       2.0,  0.0,   2.0,  false
```
Spacing and displaying decimals for round values do not matter.
Row ordering also does not matter.

### Precision
You can assume a precision of four places past the decimal and should output values with the
same level of precision.

### Assumptions
* The client has a single asset account. All transactions are to and from this single asset account.
* There are multiple clients. Transactions reference clients. If a client doesn't exist create a new record.
* Clients are represented by u16 integers. No names, addresses, or complex client profile info.

### Transactions

**Deposit**  
A deposit is a credit to the client's asset account, meaning it should increase
the available and total funds of the client account.
```
type,       client, tx, amount
deposit,    1,      1,  1.0
```

**Withdrawal**  
A withdrawal is a debit to the client's asset account,
meaning it should decrease the available and total funds of the client account.
```
type,       client, tx, amount
withdrawal, 2,      2,  1.0
```
If a client does not have sufficient available funds the withdrawal
should fail and the total amount of funds should not change.

**Dispute**  
A dispute represents a client's claim that a transaction was erroneous and should be reversed.
The transaction shouldn't be reversed yet but the associated funds should be held. This means
that the clients available funds should decrease by the amount disputed, their held funds should
increase by the amount disputed, while their total funds should remain the same.
```
type,       client, tx, amount
dispute,    1,      1,  
```

Notice that a dispute does not state the amount disputed.
Instead, a dispute references the transaction that is disputed by ID.
If the tx specified by the dispute doesn't exist you can ignore it 
and assume this is an error on our partners side.

**Resolve**  
A resolve represents a resolution to a dispute, releasing the associated held funds.
Funds that were previously disputed are no longer disputed.
This means that the clients held funds should decrease by the amount no longer disputed,
their available funds should increase by the amount no longer disputed,
and their total funds should remain the same.

```
type,       client, tx, amount
resolve,    1,      1,
```

Like disputes, resolves do not specify an amount.
Instead, they refer to a transaction that was under dispute by ID.
If the tx specified doesn't exist, or the tx isn't under dispute,
you can ignore the resolve and assume this is an error on our partner's side.

**Chargeback**  
A chargeback is the final state of a dispute and represents the client reversing a transaction.
Funds that were held have now been withdrawn.
This means that the clients held funds and total funds should decrease by the amount previously disputed.
If a chargeback occurs the client's account should be immediately frozen.

```
type,       client, tx, amount
chargeback, 1,      1,
```

Like a dispute and a resolve a chargeback refers to the transaction by ID (tx) and does not specify an amount.
Like a resolve, if the tx specified doesn't exist, or the tx isn't under dispute,
you can ignore chargeback and assume this is an error on our partner's side.
