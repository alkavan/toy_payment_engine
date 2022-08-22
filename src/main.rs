mod context;
mod data;
mod roles;
mod utility;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use crate::context::{BankContext, CsvExportContext, CsvImportContext};
use crate::data::{Account, Transaction, TransactionType};
use crate::roles::TransactionProcessor;

use crate::utility::{find_transaction_parent, is_special_transaction};
use std::collections::HashMap;
use std::env;
use std::process;

fn main() {
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // check at least one argument was passed
    if args.len() < 2 {
        process::exit(1);
    }

    // prepare csv import context
    let csv_arg = args[1].as_str();
    let csv_import = CsvImportContext::new(csv_arg);

    // read csv file with transactions
    let transactions_result = csv_import.read();

    // check we were able to read the transactions
    if let Err(err) = transactions_result {
        error!("{}", err);
        process::exit(1);
    }

    // create a container for accounts, and pass a mutable ref to the bank context
    let mut accounts: HashMap<u16, Account> = HashMap::new();
    let mut bank = BankContext::new(&mut accounts);

    let transactions = transactions_result.unwrap();

    for transaction in transactions.iter() {
        let mut parent: Option<&Transaction> = None;
        if is_special_transaction(transaction) {
            // this is not very efficient, but it is simple
            parent = find_transaction_parent(transaction, transactions.iter());
        }
        bank.process(transaction, parent);
    }

    // initializer CSV header and writer
    let csv_export_header = vec!["client", "available", "held", "total", "locked"];
    let csv_export = CsvExportContext::new(csv_export_header);

    // check for error during write
    if let Err(err) = csv_export.write(&accounts) {
        error!("{}", err);
        process::exit(1);
    }
}
