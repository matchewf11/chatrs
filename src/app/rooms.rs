use crate::app::auth;
use axum::{Extension, Json, Router, debug_handler, extract::State, middleware, routing};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

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
struct PostResponse {
    message: String,
    name: String,
    created_at: String,
}

#[debug_handler]
async fn post(
    State(pool): State<SqlitePool>,
    Extension(user_id): Extension<i64>,
    Json(data): Json<PostRequest>,
) -> Json<PostResponse> {
    // TODO: use transaction

    let sql_1 = r"
        INSERT INTO rooms (name, created_by)
        VALUES (?, ?)
        RETURNING id, created_at
        ";

    let row = sqlx::query(sql_1)
        .bind(data.name.clone())
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let room_id: i64 = row.get("id");
    let created_at = row.get("created_at");
    let sql_2 = r"
        INSERT INTO room_members (user_id, room_id)
        VALUES (?, ?)
        ";

    sqlx::query(sql_2)
        .bind(user_id)
        .bind(room_id)
        .execute(&pool)
        .await
        .unwrap();

    Json::from(PostResponse {
        message: "Successfully Posted Room".to_string(),
        name: data.name,
        created_at,
    })
}
