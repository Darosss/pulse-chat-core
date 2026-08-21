use axum::{
    Json,
    extract::{Path, Query, State, WebSocketUpgrade},
    response::Response,
};
use axum_auth::AuthBearer;
use serde::{Deserialize, Serialize};

use crate::{
    app_error::AppError,
    app_state::AppState,
    pb::message::{HistoryRequest, MessageItem},
    utils::get_token_data,
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

#[derive(Serialize, Deserialize)]
pub struct CreateMessageBody {
    pub content: String,
}

#[derive(Deserialize)]
pub struct WsAuthQuery {
    pub token: String,
}

pub async fn create_message(
    State(state): State<AppState>,
    Path(channel_id): Path<i32>,
    AuthBearer(token): AuthBearer,
    Json(payload): Json<CreateMessageBody>,
) -> Result<Json<bool>, AppError> {
    let user_data = get_token_data(state.accounts, token).await?;

    state
        .messages
        .create_message(channel_id, user_data.user_id, payload)
        .await?;

    Ok(Json(true))
}

pub async fn get_messages(
    State(state): State<AppState>,
    Path(channel_id): Path<i32>,
    Query(query): Query<GetMessagesQuery>,
    AuthBearer(token): AuthBearer,
) -> Result<Json<Vec<MessageItemResponse>>, AppError> {
    let user_data = get_token_data(state.accounts, token).await?;

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
    Query(query): Query<WsAuthQuery>,
    ws: WebSocketUpgrade,
) -> Result<Response, AppError> {
    let user_data: crate::pb::auth::ValidateTokenResponse =
        get_token_data(state.accounts, query.token).await?;

    Ok(ws.on_upgrade(move |socket| {
        state
            .messages
            .clone()
            .handle_socket(socket, user_data.user_id, channel_id)
    }))
}
