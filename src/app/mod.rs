use axum::Router;
use sqlx::SqlitePool;

mod chats;
mod users;

// can do get(_).post(_)
// make /users post
pub fn new(_pool: SqlitePool) -> Router {
    Router::merge(chats::router(), users::router())
}
