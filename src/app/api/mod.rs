use axum::{Json, Router};
use serde_json::json;
use sqlx::SqlitePool;

mod auth;
mod chats;
mod rooms;
mod sessions;
mod users;

pub fn new(pool: SqlitePool) -> Router {
    Router::merge(
        Router::merge(rooms::router(pool.clone()), sessions::router(pool.clone())),
        Router::merge(chats::new(pool.clone()), users::router(pool)),
    )
    .route(
        "/api/health",
        axum::routing::get(|| async {
            Json(json!({
                "message": "Server is running",
                "status": "ok",
            }))
        }),
    )
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_new() {
        // let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        // let router = new(pool);
        // let request = Request::builder()
        //     .uri("/health")
        //     .body(Body::empty())
        //     .unwrap();
        // let response = router.oneshot(request).await.unwrap();
        // assert_eq!(response.status(), StatusCode::OK);
        // let body = response.collect().await.unwrap();
        // let json: Value = serde_json::from_slice(&body.to_bytes()).unwrap();
        // assert_eq!(json["status"], "ok");
        // assert_eq!(json["message"], "Server is running");
        panic!();
    }
}
