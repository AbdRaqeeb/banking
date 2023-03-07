use std::io::{self, Write};
use anyhow::Result;
use rand::Rng;

pub mod account;

pub fn print_message(message: &str) -> Result<()> {
    log::info!("Printing message to stdout");

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", message)?;

    log::info!("Printed message successfully");

    Ok(())
}

pub fn generate_acct_number() -> u32 {
    let random_number = rand::thread_rng().gen_range(100000..=500000).to_string();
    let acct_number = format!("10{}", &random_number);

    acct_number.parse::<u32>().unwrap()
}

pub fn continue_transaction() -> Result<String> {
    print_message("\nWould you like to perform another transaction? (yes / no)\n")?;

    let mut response = String::new();

    read_input(&mut response)?;

    Ok(response)
}

pub fn read_input(value: &mut String) -> Result<()> {
    io::stdin()
        .read_line( value)?;

    Ok(())
}

pub fn read_number_input(value: &mut String) -> u32 {
    io::stdin()
        .read_line( value).expect("Error reading user input");

    value.trim().parse::<u32>().expect("Error reading user input")
}