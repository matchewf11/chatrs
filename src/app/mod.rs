use axum::{
    Router,
    routing::{get, post},
};
use sqlx::SqlitePool;

mod chats;
mod users;

pub fn new(_pool: SqlitePool) -> Router {
    // can do get(_).post(_)
    // make /users post
    Router::new()
        .route("/chats", get(chats::get))
        .route("/chats", post(chats::post))
        .route("/users", post(users::post))
}
