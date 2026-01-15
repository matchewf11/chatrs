use axum::Router;
use sqlx::SqlitePool;

mod auth;
mod chats;
mod rooms;
mod sessions;
mod users;

pub fn router(pool: SqlitePool) -> Router {
    Router::merge(
        Router::merge(rooms::router(pool.clone()), sessions::router(pool.clone())),
        Router::merge(chats::router(pool.clone()), users::router(pool)),
    )
}
