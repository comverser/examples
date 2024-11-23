use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub code: String,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub room_id: String,
    pub user_id: String,
}

#[derive(Serialize, Clone)]
pub struct Response {
    pub room_id: String,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: String,
    pub sender: tokio::sync::mpsc::UnboundedSender<axum::extract::ws::Message>,
}
