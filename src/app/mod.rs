use axum::Router;
use sqlx::SqlitePool;

mod api;

pub fn new(pool: SqlitePool) -> Router {
    api::router(pool)
}
