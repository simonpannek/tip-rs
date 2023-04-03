use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SendMessageData {
    pub title: String,
    pub description: String,
    pub color: Option<serenity::utils::Color>,
    pub announcement: bool,
    pub channel_id: Option<i64>,
    pub message_id: Option<i64>,
}
