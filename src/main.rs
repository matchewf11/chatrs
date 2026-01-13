use axum::{Router, routing::get};
use tokio::net::TcpListener;

struct ChatsGet {
    body: String,
}

async fn chats_get() {
    todo!();
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/chats", get(chats_get));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
