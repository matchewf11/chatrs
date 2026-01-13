use axum::Router;
use sqlx::SqlitePool;

mod chats;
mod rooms;
mod sessions;
mod users;

pub fn new(pool: SqlitePool) -> Router {
    Router::merge(
        Router::merge(rooms::router(pool.clone()), sessions::router(pool.clone())),
        Router::merge(chats::router(), users::router(pool)),
    )

    // can do get(_).post(_)
    // make /users post
}
