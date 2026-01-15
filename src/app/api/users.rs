use axum::{
    Json, Router, debug_handler, extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing,
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

pub fn router(pool: SqlitePool) -> axum::Router {
    Router::new()
        .route("/api/users", routing::post(post))
        .with_state(pool)
}

#[derive(Deserialize)]
struct PostRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct PostResponse {
    message: String,
    username: String,
    created_at: String,
    id: u64,
}

struct PostErr(sqlx::Error);

impl IntoResponse for PostErr {
    fn into_response(self) -> Response {
        match self.0 {
            sqlx::Error::Database(db_err) => {
                (StatusCode::BAD_REQUEST, db_err.message().to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal database error".to_string(),
            ),
        }
        .into_response()
    }
}

#[debug_handler]
async fn post(
    State(pool): State<SqlitePool>,
    Json(data): Json<PostRequest>,
) -> Result<Json<PostResponse>, PostErr> {
    let row = sqlx::query(
        r"
        INSERT INTO users (username, password)
        VALUES (?, ?)
        RETURNING id, created_at
        ",
    )
    .bind(data.username.clone())
    .bind(data.password)
    .fetch_one(&pool)
    .await
    .map_err(PostErr)?;

    let id = row.get("id");
    let created_at = row.get("created_at");

    Ok(Json::from(PostResponse {
        message: "Successfully Posted User".to_string(),
        username: data.username,
        created_at,
        id,
    }))
}
