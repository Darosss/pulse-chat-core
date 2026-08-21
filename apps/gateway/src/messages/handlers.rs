use axum::{
    Json,
    extract::{Path, Query, State, WebSocketUpgrade},
    response::Response,
};
use axum_auth::AuthBearer;
use serde::{Deserialize, Serialize};
use tonic::Status;

use crate::{
    accounts::ValidateTokenBody,
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
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub timestamp: i64,
}

impl From<MessageItem> for MessageItemResponse {
    fn from(value: MessageItem) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            content: value.content,
            timestamp: value.timestamp,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateMessageBody {
    pub user_id: i32,
    pub content: String,
}

pub async fn create_message(
    State(state): State<AppState>,
    Path(channel_id): Path<i32>,
    Json(payload): Json<CreateMessageBody>,
) -> Result<Json<bool>, AppError> {
    state.messages.create_message(channel_id, payload).await?;

    Ok(Json(true))
}

pub async fn get_messages(
    State(state): State<AppState>,
    Path(channel_id): Path<i32>,
    Query(query): Query<GetMessagesQuery>,
    AuthBearer(token): AuthBearer,
) -> Result<Json<Vec<MessageItemResponse>>, AppError> {
    if token.trim() == "" {
        return Err(AppError::MessageService(Status::unauthenticated(
            "You are not allowed to view messages of that channel",
        )));
    }
    let user_data = state
        .accounts
        .validate_token(ValidateTokenBody { token })
        .await?;
    if !user_data.is_valid {
        return Err(AppError::MessageService(Status::unauthenticated(
            "Your token expired. Please log-in again",
        )));
    }
    let result = state
        .messages
        .clone()
        .get_history(
            HistoryRequest {
                channel_id,
                limit: query.limit.unwrap_or(50),
            },
            user_data.user_id,
        )
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
    Path(channel_id): Path<i32>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| state.messages.clone().handle_socket(socket, channel_id))
}
