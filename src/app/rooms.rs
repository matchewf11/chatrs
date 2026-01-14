use crate::app::auth;
use axum::{Extension, Json, Router, debug_handler, extract::State, middleware, routing};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/rooms", routing::post(post))
        .route_layer(middleware::from_fn_with_state(pool.clone(), auth::auth))
        .with_state(pool)
}

#[derive(Deserialize)]
struct PostRequest {
    name: String,
}

#[derive(Serialize)]
struct PostResponse;

#[debug_handler]
async fn post(
    State(pool): State<SqlitePool>,
    Extension(user_id): Extension<u64>,
    Json(data): Json<PostRequest>,
) -> Json<PostResponse> {
    let sql_1 = r"
        INSERT INTO rooms (name, created_by)
        VALUES (?, ?)
        ";
    let sql_2 = r"
        INSERT INTO room_members (user_id, room_id)
        ";

    todo!();
}
