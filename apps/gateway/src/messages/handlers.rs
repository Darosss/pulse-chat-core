use axum::{
    Json,
    extract::{Path, Query, State, WebSocketUpgrade},
    response::Response,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_error::AppError,
    app_state::AppState,
    pb::message::{HistoryRequest, MessageItem},
};
#[derive(Deserialize)]
pub struct GetMessagesQuery {
    pub limit: Option<u32>,
    pub page: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageItemResponse {
    pub message_id: String,
    pub user_id: String,
    pub content: String,
    pub timestamp: i64,
}

impl From<MessageItem> for MessageItemResponse {
    fn from(value: MessageItem) -> Self {
        Self {
            message_id: value.message_id,
            user_id: value.user_id,
            content: value.content,
            timestamp: value.timestamp,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateMessageBody {
    pub user_id: String,
    pub content: String,
}

pub async fn create_message(
    State(state): State<AppState>,
    Path(channel_id): Path<String>,
    Json(payload): Json<CreateMessageBody>,
) -> Result<Json<bool>, AppError> {
    state.messages.create_message(channel_id, payload).await?;

    Ok(Json(true))
}

pub async fn get_messages(
    State(state): State<AppState>,
    Path(channel_id): Path<String>,
    Query(query): Query<GetMessagesQuery>,
) -> Result<Json<Vec<MessageItemResponse>>, AppError> {
    let result = state
        .messages
        .clone()
        .get_history(HistoryRequest {
            channel_id,
            limit: query.limit.unwrap_or(50),
        })
        .await?;
    let messages = result
        .messages
        .into_iter()
        .map(MessageItemResponse::from)
        .collect();

    Ok(Json(messages))
}

pub async fn chat_socket(
    State(state): State<AppState>,
    Path(channel_id): Path<String>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| state.messages.clone().handle_socket(socket, channel_id))
}
