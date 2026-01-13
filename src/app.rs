use axum::{
    Json, Router, debug_handler,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

pub fn new() -> Router {
    // can do get(_).post(_)
    // make /users post
    Router::new()
        .route("/chats", get(chats_get))
        .route("/chats", post(chat_post))
}

#[derive(Serialize)]
struct ChatsGet {
    body: String,
}

#[debug_handler]
async fn chats_get() -> Json<ChatsGet> {
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
async fn chat_post(Json(payload): Json<ChatPost>) -> Json<ChatPostReturn> {
    println!("[POST] /chats");
    Json::from(ChatPostReturn { body: payload.body })
}
