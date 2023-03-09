use rusqlite::{self, Connection, Result};

#[allow(dead_code)]
pub struct Transaction<'a> {
    id: Option<u32>,
    from_account_number: Option<&'a u32>,
    to_account_number: Option<&'a u32>,
    amount: &'a u32,
    transaction_type: String,
    created_at: Option<String>,
}

impl<'a> Transaction<'a> {
    pub fn new(
        from_account_number: Option<&'a u32>,
        to_account_number: Option<&'a u32>,
        amount: &'a u32,
        transaction_type: String,
    ) -> Transaction<'a> {
        Transaction {
            id: None,
            from_account_number,
            to_account_number,
            amount,
            transaction_type,
            created_at: None,
        }
    }
}

pub fn create_deposit_transaction(conn: &Connection, transaction: Transaction) -> Result<()> {
    let mut statement = conn.prepare(
        "
                INSERT INTO transactions (to_account_number, amount, transaction_type)
                VALUES (?1, ?2, ?3);
            "
    )?;

    statement.execute(
        (&transaction.to_account_number.unwrap(), &transaction.amount, &transaction.transaction_type)
    )?;

    Ok(())
}

pub fn create_withdraw_transaction(conn: &Connection, transaction: Transaction) -> Result<()> {
    let mut statement = conn.prepare(
        "
                INSERT INTO transactions (from_account_number, amount, transaction_type)
                VALUES (?1, ?2, ?3);
            "
    )?;

    statement.execute(
        (&transaction.from_account_number.unwrap(), &transaction.amount, &transaction.transaction_type)
    )?;

    Ok(())
}

pub fn create_transfer_transaction(tx: &rusqlite::Transaction, transaction: Transaction) -> Result<()> {
    let mut statement = tx.prepare(
        "
                INSERT INTO transactions (from_account_number, to_account_number, amount, transaction_type)
                VALUES (?1, ?2, ?3, ?4);
            "
    )?;

    statement.execute(
        (
            &transaction.from_account_number.unwrap(),
            &transaction.to_account_number.unwrap(),
            &transaction.amount,
            &transaction.transaction_type
        )
    )?;

    Ok(())
}
