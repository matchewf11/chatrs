use axum::{Json, Router, debug_handler};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/chats", axum::routing::get(get))
        .route("/chats", axum::routing::post(post))
}

#[derive(Serialize)]
struct ChatsGet {
    body: String,
}

#[debug_handler]
async fn get() -> Json<ChatsGet> {
    println!("[GET] /chats");
    Json::from(ChatsGet {
        body: "This is the first message".to_string(),
    })
}

#[derive(Deserialize)]
struct ChatPost {
    body: String,
}

#[derive(Serialize)]
struct ChatPostReturn {
    body: String,
}

#[debug_handler]
async fn post(Json(payload): Json<ChatPost>) -> Json<ChatPostReturn> {
    println!("[POST] /chats");
    Json::from(ChatPostReturn { body: payload.body })
}
