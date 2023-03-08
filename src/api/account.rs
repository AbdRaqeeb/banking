use rusqlite::{Connection, Transaction};
use anyhow::Result;
use crate::db;
use crate::api;
use crate::db::account::Account;

pub fn create_account(conn: &Connection) -> Result<String> {
    crate::api::print_message("Please enter your email")?;

    let mut email = String::new();

    api::read_input(&mut email)?;

    let account_number = api::generate_acct_number();

    let result = db::user::get_user(conn, &email);

    let user;

    if let Some(Ok(value)) = result {
        user = value
    } else {
        user = db::user::create_user(conn, &email).unwrap().unwrap();
    };

    let account = Account::new(user.get_id(), account_number);

    if let Some(Ok(account)) = db::account::create_account(conn, &account) {
        api::print_message(format!("Dear {}\nYour account number is {}", user.get_email(), account.get_account_number()).as_str())?;
    } else {
        api::print_message("Error creating an account")?;
    };


    api::continue_transaction()
}

pub fn deposit_money(conn: &Connection) -> Result<String> {
    log::info!("[API] [DEPOSIT] depositing money to account");

    let account = get_account(conn, "\nPlease enter your account number").expect("\n");

    log::info!("[API] [DEPOSIT] Retrieving deposit amount");

    let amount = retrieve_amount();

    log::info!("[API] [DEPOSIT] updating account balance");

    let new_balance = &account.get_account_balance() + &amount;

    if let Some(Ok(account)) = db::account::update_account(conn, &account.get_account_number(), &new_balance) {
        api::print_message(
            format!(
                "\nThe money was deposited successfully.\n\nThe account details below:\n\nAccount Number: {}\nBalance {}",
                account.get_account_number(),
                account.get_account_balance(),
            ).as_str()
        )?;
    } else {
        api::print_message("Error depositing money into account")?;
    };

    api::continue_transaction()
}

pub fn get_account_balance(conn: &Connection) -> Result<String> {
    let account = get_account(conn, "\nPlease enter your account number").expect("\n");

    api::print_message(
        format!(
            "\nYour account balance:\n\nBalance {} \nAccount Number: {}",
            account.get_account_balance(),
            account.get_account_number(),
        ).as_str()
    )?;

    api::continue_transaction()
}

pub fn withdraw_from_account(conn: &Connection) -> Result<String> {
    log::info!("[API] [WITHDRAW_FROM_ACCOUNT] withdrawing from account");

    let account = get_account(conn, "\nPlease enter your account number").expect("\n");

    let amount = retrieve_amount();

    log::info!("[API] [WITHDRAW_FROM_ACCOUNT] updating account balance");

    if &account.get_account_balance() < &amount {
        log::info!("[API] [WITHDRAW_FROM_ACCOUNT] insufficient funds");

        api::print_message("\nInsufficient funds\n")?;

        return api::continue_transaction();
    }

    let new_balance = &account.get_account_balance() - &amount;

    if let Some(Ok(account)) = db::account::update_account(conn, &account.get_account_number(), &new_balance) {
        api::print_message(
            format!(
                "\nThe withdrawal was successful.\n\nThe account details below:\n\nAccount Number: {}\nBalance {}",
                account.get_account_number(),
                account.get_account_balance(),
            ).as_str()
        )?;
    } else {
        api::print_message("Error withdrawing money from account")?;
    };

    api::continue_transaction()
}

pub fn transfer_money(conn: &mut Connection) -> Result<String> {
    log::info!("[API] [TRANSFER_MONEY] creating transaction");

    let tx = conn.transaction()?;

    log::info!("[API] [TRANSFER_MONEY] retrieving from_account");

    let from_account = get_account_with_transaction(&tx, "\nPlease enter your account number").expect("\n");

    log::info!("[API] [TRANSFER_MONEY] retrieving amount");

    let amount = retrieve_amount();

    log::info!("[API] [TRANSFER_MONEY] retrieving recipient account");

    let to_account = get_account_with_transaction(&tx, "\nPlease enter the recipient account number").expect("\n");


    if &from_account.get_account_balance() < &amount {
        log::info!("[API] [TRANSFER_MONEY] insufficient funds");

        api::print_message("\nInsufficient funds\n")?;

        return api::continue_transaction();
    }

    log::info!("[API] [TRANSFER_MONEY] deducting amount from the from_account");

    let from_account_new_balance = &from_account.get_account_balance() - &amount;

    db::account::update_account_with_transaction(&tx, &from_account.get_account_number(), &from_account_new_balance)?;

    let to_account_new_balance = &to_account.get_account_balance() + &amount;

    db::account::update_account_with_transaction(&tx, &to_account.get_account_number(), &to_account_new_balance)?;

    let from_account = db::account::get_account_with_transaction(&tx, &from_account.get_account_number())
        .expect("Error during transfer!!!")
        .expect("Error during transfer!!!");

    let to_account = db::account::get_account_with_transaction(&tx, &to_account.get_account_number())
        .expect("Error during transfer!!!")
        .expect("Error during transfer!!!");

    tx.commit()?;

    api::print_message(
        format!(
            "\nThe transfer was successful.\n\nThe account details below:\n\nSender Account:\r\n\r\nAccount Number: {}\nBalance {}\n\nRecipient Account:\r\n\r\nAccount Number: {}\nBalance {}",
            from_account.get_account_number(),
            from_account.get_account_balance(),
            to_account.get_account_number(),
            to_account.get_account_balance(),
        ).as_str()
    )?;

    api::continue_transaction()
}

pub fn get_account(conn: &Connection, message: &str) -> Option<Account> {
    let account_number = retrieve_account_number(message);

    if let Some(Ok(account)) = db::account::get_account(conn, &account_number) {
        Some(account)
    } else {
        log::info!("[API] [GET_ACCOUNT] invalid account balance");
        api::print_message("\nThe account number is invalid").expect("Application error");
        None
    }
}

pub fn get_account_with_transaction(tx: &Transaction, message: &str) -> Option<Account> {
    let account_number = retrieve_account_number(message);

    if let Some(Ok(account)) = db::account::get_account_with_transaction(tx, &account_number) {
        Some(account)
    } else {
        log::info!("[API] [GET_ACCOUNT_WITH_TRANSACTION] invalid account balance");
        api::print_message("\nThe account number is invalid").expect("Application error");
        None
    }
}

pub fn retrieve_account_number(message: &str) -> u32 {
    let mut account_number = String::new();

    api::print_message(message).expect("Application error");

    log::info!("[API] [RETRIEVE_ACCOUNT_NUMBER] retrieving account number from input");
    api::read_number_input(&mut account_number)
}

pub fn retrieve_amount() -> u32 {
    let mut amount = String::new();

    api::print_message("\nPlease enter the amount").expect("Application error");

    log::info!("[API] [RETRIEVE_AMOUNT] retrieving amount from input");

    api::read_number_input(&mut amount)
}