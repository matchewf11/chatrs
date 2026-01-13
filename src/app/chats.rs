use axum::{Json, debug_handler};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatsGet {
    body: String,
}

#[debug_handler]
pub async fn get() -> Json<ChatsGet> {
    println!("[GET] /chats");
    Json::from(ChatsGet {
        body: "This is the first message".to_string(),
    })
}

#[derive(Deserialize)]
pub struct ChatPost {
    body: String,
}

#[derive(Serialize)]
pub struct ChatPostReturn {
    body: String,
}

#[debug_handler]
pub async fn post(Json(payload): Json<ChatPost>) -> Json<ChatPostReturn> {
    println!("[POST] /chats");
    Json::from(ChatPostReturn { body: payload.body })
}
