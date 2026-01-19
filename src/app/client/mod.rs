use axum::{Router, response::Html, routing};

pub fn new() -> Router {
    Router::new().route(
        "/",
        routing::get(|| async { Html(include_str!("root.html")) }),
    )
}
