#[macro_use]
extern crate log;

use std::path::Path;
use anyhow::{Context, Result};

pub mod db;
pub mod api;

fn main() -> Result<()> {
    env_logger::init();

    info!("starting up the application");

    let path = Path::new("Dbank.db");

    let connection = db::connect(path)
        .with_context(||{
            log::error!("connection error to database: {}", path.display());
            format!("connection to database error")
        })?;


    bank::run(&connection)
        .with_context(||format!("Application error"))?;
    Ok(())
}
