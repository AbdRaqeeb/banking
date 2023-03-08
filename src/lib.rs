use anyhow::Result;
use log::error;
use rusqlite::Connection;

pub mod db;
pub mod api;

pub fn run(mut conn: Connection) -> Result<()> {
    api::print_message("Welcome to MABJ Financial\n")?;

    let message = "
Please enter the number of the operation you would like to perform:\n
1. Create an account
2. Deposit money
3. Withdraw money
4. Transfer money
5. Account balance
6. Quit
";

    loop {
        let mut choice = String::new();

        api::print_message(message)?;

        api::read_input(&mut choice)?;

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                error!("Error parsing choice:\n {}", err);
                continue
            },
        };

        let repeat;

        match choice {
            1 => {
                repeat = api::account::create_account(&conn)?;
            },
            2 => {
                repeat = api::account::deposit_money(&conn)?
            },
            3 => {
                repeat = api::account::withdraw_from_account(&conn)?
            },
            4 => {
                repeat = api::account::transfer_money(&mut conn)?
            },
            5 => {
                repeat = api::account::get_account_balance(&conn)?
            },
            6 => {
                api::print_message("\nThank you for banking with us!!!\n").expect("Application error");
                break;
            },
            _  => {
                api::print_message("\nInvalid option selected. Please try again!!!\n").expect("Application error");
                continue
            },
        }

        if repeat.trim().ne("yes") {
            break;
        }
    }

    Ok(())
}

