use anyhow::Result;
use log::error;
use rusqlite::Connection;

pub mod db;
pub mod api;

pub fn run(conn: &Connection) -> Result<()> {
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
                repeat = api::account::create_account(conn)?;
            },
            2 => {
                repeat = api::account::deposit_money(conn)?
            }
            _  => break,
        }

        if repeat.trim().ne("yes") {
            break;
        }
    }

    Ok(())
}

