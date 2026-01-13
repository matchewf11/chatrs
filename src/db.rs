use sqlx::SqlitePool;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct FailedConnection;
impl Error for FailedConnection {}
impl Display for FailedConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to Connect to Db")
    }
}

pub async fn new() -> Result<SqlitePool, FailedConnection> {
    // connect_with for options
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .map_err(|_| FailedConnection)?;

    sqlx::query(
        r"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            last_active TEXT,
            CHECK (length(username) > 2)
        )
        ",
    )
    .execute(&pool)
    .await
    .unwrap();
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
    .unwrap();
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
    .unwrap();
    sqlx::query(
        r"
        CREATE TABLE IF NOT EXISTS room_members (
            user_id INTEGER NOT NULL,
            room_id INTEGER NOT NULL,
            joined_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (user_id, room_id),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (room_id) REFERENCES rooms(id) ON DELETE CASCADE
        )
        ",
    )
    .execute(&pool)
    .await
    .unwrap();
    Ok(pool)
}
