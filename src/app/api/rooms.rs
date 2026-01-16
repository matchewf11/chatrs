use crate::app::api::auth;
use axum::{
    Extension, Json, Router, debug_handler,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing,
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/rooms", routing::post(post))
        .route("/api/rooms/{id}", routing::post(join))
        .route("/api/rooms", routing::get(get))
        .route_layer(middleware::from_fn_with_state(pool.clone(), auth::auth))
        .with_state(pool)
}

#[derive(Serialize)]
struct GetRow {
    id: i64,
    name: String,
    created_by: i64,
    created_at: String,
}
struct GetErr;
impl IntoResponse for GetErr {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
    }
}

#[debug_handler]
async fn get(State(pool): State<SqlitePool>) -> Result<Json<Vec<GetRow>>, GetErr> {
    // Extension(user_id): Extension<i64>,
    // add if this user has permissions to that room
    let sql = "SELECT id, name, created_by, created_at FROM rooms";
    let get_rows = sqlx::query(sql)
        .fetch_all(&pool)
        .await
        .map_err(|_| GetErr)?
        .iter()
        .map(|row| GetRow {
            id: row.get("id"),
            name: row.get("name"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
        })
        .collect();
    Ok(Json(get_rows))
}

struct JoinErr(sqlx::Error);
impl IntoResponse for JoinErr {
    fn into_response(self) -> Response {
        let err = self.0;
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("internal server error: {}", err),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct JoinResponse {
    message: String,
}

#[debug_handler]
async fn join(
    State(pool): State<SqlitePool>,
    Extension(user_id): Extension<i64>,
    Path(room_id): Path<i64>,
) -> Result<Json<JoinResponse>, JoinErr> {
    sqlx::query(
        r"
        INSERT INTO room_members (user_id, room_id)
        VALUES (?, ?)
        ",
    )
    .bind(user_id)
    .bind(room_id)
    .execute(&pool)
    .await
    .map_err(JoinErr)?;

    Ok(Json::from(JoinResponse {
        message: format!("Joined Room: {}", room_id),
    }))
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

struct PostErr(sqlx::Error);

impl IntoResponse for PostErr {
    fn into_response(self) -> Response {
        let err = self.0;
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("internal server error: {}", err),
        )
            .into_response()
    }
}

#[debug_handler]
async fn post(
    State(pool): State<SqlitePool>,
    Extension(user_id): Extension<i64>,
    Json(data): Json<PostRequest>,
) -> Result<Json<PostResponse>, PostErr> {
    let mut tx = pool.begin().await.map_err(PostErr)?;

    let sql_1 = r"
        INSERT INTO rooms (name, created_by)
        VALUES (?, ?)
        RETURNING id, created_at
        ";

    let row = sqlx::query(sql_1)
        .bind(data.name.clone())
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(PostErr)?;

    let room_id: i64 = row.get("id");
    let created_at = row.get("created_at");
    let sql_2 = r"
        INSERT INTO room_members (user_id, room_id)
        VALUES (?, ?)
        ";

    sqlx::query(sql_2)
        .bind(user_id)
        .bind(room_id)
        .execute(&mut *tx)
        .await
        .map_err(PostErr)?;

    tx.commit().await.map_err(PostErr)?;

    Ok(Json::from(PostResponse {
        message: "Successfully Posted Room".to_string(),
        name: data.name,
        created_at,
    }))
}
