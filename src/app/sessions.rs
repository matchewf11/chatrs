use axum::{Json, Router, debug_handler, extract::State, routing};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/sessions", routing::post(post))
        .with_state(pool)
}

#[derive(Deserialize)]
struct PostRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct PostResponse {
    token: String,
    expires_at: String,
}

#[debug_handler]
async fn post(State(pool): State<SqlitePool>, Json(data): Json<PostRequest>) -> Json<PostResponse> {
    let sql = r"
        INSERT INTO sessions (token, user_id)
        VALUES (
            ?,
            (SELECT id FROM users WHERE username = ? AND password = ?)
        )
        RETURNING token, expires_at
        ";
    let token = "foo";

    let row = sqlx::query(sql)
        .bind(token)
        .bind(data.username)
        .bind(data.password)
        .fetch_one(&pool)
        .await
        .unwrap();
    let token = row.get("token");
    let expires_at = row.get("expires_at");
    Json::from(PostResponse { token, expires_at })
}
