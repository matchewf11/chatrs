use axum::{Router, response::Html, routing};

pub fn new() -> Router {
    Router::new().route("/", routing::get(root))
}

async fn root() -> Html<&'static str> {
    Html(include_str!("root.html"))
}
