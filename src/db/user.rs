use rusqlite::{Connection, Result};

pub struct User {
    id: u32,
    email: String,
}

impl User {
    pub fn get_id(&self) -> u32 { self.id }

    pub fn get_email(&self) -> &str {
        self.email.as_str()
    }
}

pub fn create_user(conn: &Connection, email: &str) -> Option<Result<User>> {
    log::info!("[DATABASE] preparing query to create user");

    let mut statement = conn.prepare(
        "INSERT INTO users (email) VALUES (:email)"
    ).ok()?;

    log::info!("[DATABASE] executing query to create user");

    statement.execute(&[(":email", email)]).ok()?;

    log::info!("[DATABASE] getting created user");
    get_user(conn, email)
}

pub fn get_user(conn: &Connection, email: &str) -> Option<Result<User>> {
    log::info!("[DATABASE] preparing query to get user");

    let mut statement = conn.prepare(
        "SELECT id, email FROM users WHERE email = ?"
    ).ok()?;

    log::info!("[DATABASE] querying users");

    let mut user_iter = statement.query_map([email], |row| {
        log::info!("[DATABASE] mapping user rows");

        Ok(User {
            id: row.get(0)?,
            email: row.get(1)?,
        })
    }).ok()?;

    log::info!("[DATABASE] returning users");
    user_iter.next()
}

