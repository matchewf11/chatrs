//! Entry point to the api module.

use axum::{Json, Router, routing};
use serde_json::json;
use sqlx::SqlitePool;

/// Provides auth middleware.
mod auth;

/// Provides /chats api.
mod chats;

/// Provides /rooms api.
mod rooms;

/// Provides /sessions api.
mod sessions;

/// Provides /users api.
mod users;

/// Creates a new router that handles the /api/... endpoints.
pub fn new(pool: SqlitePool) -> Router {
    Router::merge(
        Router::merge(rooms::router(pool.clone()), sessions::new(pool.clone())),
        Router::merge(chats::new(pool.clone()), users::router(pool)),
    )
    .route(
        "/api/health",
        routing::get(|| async {
            Json(json!({
                "message": "Server is running",
                "status": "ok",
            }))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, extract::Request, http::StatusCode};
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_new() {
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        let router = new(pool);
        let request = Request::builder()
            .uri("/api/health")
            .body(Body::empty())
            .unwrap();
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.collect().await.unwrap();
        let json: Value = serde_json::from_slice(&body.to_bytes()).unwrap();

        assert_eq!(json["status"], "ok");
        assert_eq!(json["message"], "Server is running");
    }
}
