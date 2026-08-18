use axum::{
    Json,
    extract::{Path, Query, State},
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
