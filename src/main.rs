use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "hello world" }));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Running on port 3000");
    axum::serve(listener, app).await.unwrap();
}
