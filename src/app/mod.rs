//! Entry point to the app module.
//! Includes both the api and the client.

use axum::{Json, Router, routing};
use serde_json::json;
use sqlx::SqlitePool;

/// Handles router of api code.
mod api;

/// Handles router of client code.
mod client;

/// Returns a router to manage the entire app.
pub fn new(pool: SqlitePool) -> Router {
    Router::merge(api::new(pool), client::new()).route(
        "/health",
        routing::get(|| async {
            Json(json!({
                "status": "ok",
                "message": "Server is running",
            }))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_new() {
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        let router = new(pool);
        let request = Request::builder()
            .uri("/health")
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
