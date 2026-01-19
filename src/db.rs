//! Sets up a database connection and initialization.

use sqlx::SqlitePool;
use std::error;
use std::fmt::{self, Display, Formatter};

/// Error value for initializing the database.
#[derive(Debug)]
pub enum Error {
    /// Could not open the database.
    FailedConnection,
    /// Error within the server.
    InternalServerErorr,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::FailedConnection => write!(f, "failed to connect to the database"),
            Error::InternalServerErorr => write!(f, "internal server error"),
        }
    }
}

/// Initializes a database, opening it
/// and creating initial tables.
///
/// # Examples
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     let _db = chatrs::db::build().await.unwrap();
/// }
/// ```
pub async fn build() -> Result<SqlitePool, Error> {
    // connect_with for options
    // let pool = SqlitePool::connect("sqlite::memory:")
    let pool = SqlitePool::connect("foo.db")
        .await
        .map_err(|_| Error::FailedConnection)?;

    sqlx::query(
        r"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            last_active TEXT,
            CHECK (length(username) > 2)
        )
        ",
    )
    .execute(&pool)
    .await
    .map_err(|_| Error::InternalServerErorr)?;

    sqlx::query(
        r"
        CREATE TABLE IF NOT EXISTS rooms (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_by INTEGER NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
        )
        ",
    )
    .execute(&pool)
    .await
    .map_err(|_| Error::InternalServerErorr)?;

    sqlx::query(
        r"
        CREATE TABLE IF NOT EXISTS chats (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            room_id INTEGER NOT NULL,
            author_id INTEGER NOT NULL,
            body TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            CHECK (length(body) > 0),
            FOREIGN KEY (room_id) REFERENCES rooms(id) ON DELETE CASCADE,
            FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
        )
        ",
    )
    .execute(&pool)
    .await
    .map_err(|_| Error::InternalServerErorr)?;

    sqlx::query(
        r"
        CREATE TABLE IF NOT EXISTS room_members (
            user_id INTEGER NOT NULL,
            room_id INTEGER NOT NULL,
            joined_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            last_active TEXT,
            PRIMARY KEY (user_id, room_id),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (room_id) REFERENCES rooms(id) ON DELETE CASCADE
        )
        ",
    )
    .execute(&pool)
    .await
    .map_err(|_| Error::InternalServerErorr)?;

    sqlx::query(
        r"
        CREATE TABLE IF NOT EXISTS sessions (
            token TEXT PRIMARY KEY,
            user_id INTEGER NOT NULL,
            expires_at TEXT NOT NULL DEFAULT (datetime(CURRENT_TIMESTAMP, '+1 hour')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
        ",
    )
    .execute(&pool)
    .await
    .map_err(|_| Error::InternalServerErorr)?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build() {
        let _r = build().await.unwrap();
    }

    #[test]
    fn test_err() {
        assert_eq!(
            format!("{}", Error::FailedConnection),
            "failed to connect to the database".to_string()
        );
        assert_eq!(
            format!("{}", Error::InternalServerErorr),
            "internal server error".to_string()
        );
    }
}
