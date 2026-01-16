use axum::Router;
use sqlx::SqlitePool;

mod api;
mod client;

pub fn new(pool: SqlitePool) -> Router {
    Router::merge(api::router(pool), client::router())
}
