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

#[derive(Deserialize, Serialize)]
pub struct CreateSurveyData {
    pub survey_options: Vec<SurveyOption>,
    pub preliminary_result: bool,
}

#[derive(Deserialize, Serialize)]
pub struct SurveyOption {
    pub label: String,
    pub question: String,
    pub response: SurveyResponse,
}

#[derive(Deserialize, Serialize)]
pub enum SurveyResponse {
    SelectMenu(Vec<String>),
    InputText(String),
}
