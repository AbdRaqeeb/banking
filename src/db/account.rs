use rusqlite::{Connection, params, Result, Transaction};

pub struct Account {
    user_id: u32,
    account_number: u32,
    balance: u32,
}

impl Account {
    pub fn new(user_id: u32, account_number: u32) -> Account {
        Account {
            user_id,
            account_number,
            balance: 0,
        }
    }

    pub fn get_account_number(&self) -> u32 {
        self.account_number
    }

    pub fn get_account_balance(&self) -> u32 {
        self.balance
    }
}

fn account_map(row: &rusqlite::Row) -> Result<Account> {
    Ok(Account {
        user_id: row.get(0)?,
        account_number: row.get(1)?,
        balance: row.get(2)?,
    })
}

pub fn get_account(conn: &Connection, account_number: &u32) -> Option<Result<Account>> {
    let mut statement = conn.prepare(
        "SELECT user_id, account_number, balance FROM accounts WHERE account_number = ?"
    ).ok()?;

    let mut account_iter = statement.query_map([account_number], account_map).ok()?;

    account_iter.next()
}

pub fn get_account_with_transaction(tx: &Transaction, account_number: &u32) -> Option<Result<Account>> {
    let mut statement = tx.prepare(
        "SELECT user_id, account_number, balance FROM accounts WHERE account_number = ?"
    ).ok()?;

    let mut account_iter = statement.query_map([account_number], account_map).ok()?;

    account_iter.next()
}

pub fn create_account(conn: &Connection, account: &Account) -> Option<Result<Account>> {
    let mut statement = conn.prepare(
        "
                INSERT INTO accounts (user_id, account_number, balance)
                VALUES (:user_id, :account_number, :balance);
            "
    ).ok()?;

    statement.execute(&[
        (":user_id", &account.user_id),
        (":account_number", &account.account_number),
        (":balance", &account.balance),
    ]).ok()?;

    get_account(conn, &account.account_number)
}

pub fn update_account(
    conn: &Connection,
    account_number: &u32,
    value: &u32,
) -> Option<Result<Account>> {

    let mut statement = conn.prepare(
        "UPDATE accounts SET balance = ? WHERE account_number = ?"
    ).ok()?;

    statement.execute(params![value, account_number]).ok()?;

    get_account(conn, account_number)
}

pub fn update_account_with_transaction(
    tx: &Transaction,
    account_number: &u32,
    value: &u32,
) -> Result<()> {
    let mut statement = tx.prepare(
        "UPDATE accounts SET balance = ? WHERE account_number = ?"
    )?;

    statement.execute(params![value, account_number])?;

    Ok(())
}
