use rusqlite::{Connection};
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
        user =  db::user::create_user(conn, &email).unwrap().unwrap();
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
    log::info!("[API] [DEPOSIT] Retrieving account number");

    let mut account_number = String::new();

    api::print_message("\nPlease enter your account number")?;

    let account_number = api::read_number_input(&mut account_number);

    if let Some(Ok(account)) = db::account::get_account(conn, &account_number) {

        log::info!("[API] [DEPOSIT] Retrieving deposit amount");

        let mut amount = String::new();

        api::print_message("\nPlease enter the amount")?;

        let amount = api::read_number_input(&mut amount);

        log::info!("[API] [DEPOSIT] updating account balance");

        if let Some(Ok(account)) = db::account::update_account(conn, &account.get_account_number(), &account.get_account_balance(), &amount) {
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


    } else {
        log::info!("[API] [DEPOSIT] invalid account balance");
        api::print_message("The account number is invalid")?;
    };


    api::continue_transaction()
}
