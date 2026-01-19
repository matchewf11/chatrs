//! Manages user sessions.

use axum::{
    Json, Router, debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing,
};
use rand::prelude::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

/// Returns a new router for the sessions api.
pub fn new(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/sessions", routing::post(post))
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

struct PostErr;
impl IntoResponse for PostErr {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
    }
}

#[debug_handler]
async fn post(
    State(pool): State<SqlitePool>,
    Json(data): Json<PostRequest>,
) -> Result<Json<PostResponse>, PostErr> {
    let sql = r"
        INSERT INTO sessions (token, user_id)
        VALUES (
            ?,
            (SELECT id FROM users WHERE username = ? AND password = ?)
        )
        RETURNING token, expires_at
        ";

    let row = sqlx::query(sql)
        .bind(generate_token())
        .bind(data.username)
        .bind(data.password)
        .fetch_one(&pool)
        .await
        .map_err(|_| PostErr)?;

    let token = row.get("token");
    let expires_at = row.get("expires_at");
    Ok(Json::from(PostResponse { token, expires_at }))
}

fn generate_token() -> String {
    let mut rng = rand::rng();
    (0..20)
        .map(|_| rng.sample(rand::distr::Alphanumeric) as char)
        .collect()
}

#[cfg(test)]
mod tests {

    fn test_new() {
        panic!();
    }

    fn test_post_err() {
        panic!();
    }
}

// comments and tests
// fix functions
// for all 4 functions
