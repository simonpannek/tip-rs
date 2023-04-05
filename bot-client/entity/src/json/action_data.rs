use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct SendMessageData {
    pub title: String,
    pub description: String,
    pub color: Option<serenity::utils::Color>,
    pub announcement: bool,
    pub channel_id: Option<i64>,
    pub message_id: Option<i64>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateSurveyData {
    pub survey_options: Vec<SurveyQuestion>,
    pub preliminary_result: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SurveyQuestion {
    pub label: String,
    pub question: String,
    pub response_options: SurveyResponseOption,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum SurveyResponseOption {
    SelectMenu(Vec<String>),
    InputText,
}
