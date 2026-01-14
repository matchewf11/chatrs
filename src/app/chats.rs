use axum::{
    Extension, Json, Router, debug_handler,
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing,
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::app::auth;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/chats", routing::post(post))
        .route_layer(middleware::from_fn_with_state(pool.clone(), auth::auth))
        .with_state(pool)
}

#[derive(Deserialize)]
struct PostRequest {
    room: String,
    body: String,
}

#[derive(Serialize)]
struct PostResponse {
    message: String,
}

enum PostErr {
    InternalServerError(sqlx::Error),
    UserNotInRoom,
}

impl IntoResponse for PostErr {
    fn into_response(self) -> Response {
        match self {
            Self::InternalServerError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("internal server error: {}", err),
            ),
            Self::UserNotInRoom => (StatusCode::UNAUTHORIZED, "not in the room".to_string()),
        }
        .into_response()
    }
}

#[debug_handler]
async fn post(
    State(pool): State<SqlitePool>,
    Extension(user_id): Extension<i64>,
    Json(data): Json<PostRequest>,
) -> Result<Json<PostResponse>, PostErr> {
    let is_user_in_room: bool = sqlx::query(
        r"
        SELECT EXISTS (
            SELECT 1
            FROM room_members
            WHERE
                room_id = (SELECT id FROM rooms WHERE name = ?) AND
                user_id = ?
        ) as is_user_in_room
        ",
    )
    .bind(data.room.clone())
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(PostErr::InternalServerError)?
    .get("is_user_in_room");
    if !is_user_in_room {
        return Err(PostErr::UserNotInRoom);
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(PostErr::InternalServerError)?;

    sqlx::query(
        r"
        INSERT INTO chats (room_id, author_id, body)
        VALUES ((SELECT id FROM rooms WHERE name = ?), ?, ?)
        ",
    )
    .bind(data.room.clone())
    .bind(user_id)
    .bind(data.body)
    .execute(&mut *tx)
    .await
    .map_err(PostErr::InternalServerError)?;

    sqlx::query(
        r"
        UPDATE users
        SET last_active = CURRENT_TIMESTAMP
        WHERE id = ?
        ",
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .map_err(PostErr::InternalServerError)?;

    sqlx::query(
        r"
        UPDATE room_members
        SET last_active = CURRENT_TIMESTAMP
        WHERE
            user_id = ? AND
            room_id = (SELECT id FROM rooms WHERE name = ?)
        ",
    )
    .bind(user_id)
    .bind(data.room)
    .execute(&mut *tx)
    .await
    .map_err(PostErr::InternalServerError)?;

    tx.commit()
        .await
        .map_err(PostErr::InternalServerError)?;

    Ok(Json::from(PostResponse {
        message: "Posted a Message".to_string(),
    }))
}
