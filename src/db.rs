pub mod user;
pub mod account;

use std::path::Path;
use rusqlite::{Connection, Result};

pub fn connect(database: &Path) -> Result<Connection> {
    log::info!("[DATABASE] Connecting to database");

    let conn = Connection::open(database)?;

    log::info!("[DATABASE] creating `users` table if not exist");
    conn.execute(
      "
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                email VARCHAR(255) NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
            );
          ",
      ()
    )?;

    log::info!("[DATABASE] creating `accounts` table if not exist");
    conn.execute(
      "
            CREATE TABLE IF NOT EXISTS accounts (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                account_number BIGINT NOT NULL,
                balance INTEGER NOT NULL DEFAULT 0,
                user_id INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,

                FOREIGN KEY (user_id)
                            REFERENCES users (id)
            );
          ",
      ()
    )?;

    log::info!("[DATABASE] connection successful");
    Ok(conn)
}
