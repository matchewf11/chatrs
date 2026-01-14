use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use sqlx::{Row, SqlitePool};

pub async fn auth(
    State(pool): State<SqlitePool>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user_id: u64 = sqlx::query(
        r"
        SELECT user_id
        FROM sessions
        WHERE token = ? AND CURRENT_TIMESTAMP < sessions.expires_at
        ",
    )
    .bind(token)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map(|r| r.get("user_id"))
    .ok_or(StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(user_id);
    Ok(next.run(req).await)
}
