pub async fn post() {}

// CREATE TABLE IF NOT EXISTS users (
//     id INTEGER PRIMARY KEY AUTOINCREMENT,
//     username TEXT NOT NULL UNIQUE,
//     password_hash TEXT NOT NULL,
//     created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     last_active TEXT,
//     CHECK (length(username) > 2)
// )
// CREATE TABLE IF NOT EXISTS rooms (
//     id INTEGER PRIMARY KEY AUTOINCREMENT,
//     name TEXT NOT NULL UNIQUE,
//     created_by INTEGER NOT NULL,
//     created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
// )
// CREATE TABLE IF NOT EXISTS chats (
//     id INTEGER PRIMARY KEY AUTOINCREMENT,
//     room_id INTEGER NOT NULL,
//     author_id INTEGER NOT NULL,
//     body TEXT NOT NULL,
//     created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     CHECK (length(body) > 0),
//     FOREIGN KEY (room_id) REFERENCES rooms(id) ON DELETE CASCADE,
//     FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
// )
// CREATE TABLE IF NOT EXISTS room_members (
//     user_id INTEGER NOT NULL,
//     room_id INTEGER NOT NULL,
//     joined_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     PRIMARY KEY (user_id, room_id),
//     FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
//     FOREIGN KEY (room_id) REFERENCES rooms(id) ON DELETE CASCADE
// )
