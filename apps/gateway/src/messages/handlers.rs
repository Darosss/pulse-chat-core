use axum::{
    Json,
    extract::{Path, Query, State},
};
use axum_auth::AuthBearer;
use serde::{Deserialize, Serialize};
use tonic::Status;

use crate::{
    app_error::AppError,
    app_state::AppState,
    pb::message::{HistoryRequest, MessageItem, message::DirectHistoryRequest},
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

pub async fn create_message(
    State(state): State<AppState>,
    Path(channel_id): Path<i32>,
    AuthBearer(token): AuthBearer,
    Json(payload): Json<CreateMessageBody>,
) -> Result<Json<bool>, AppError> {
    let user_data = get_token_data(&state, &token).await?;

    state
        .messages
        .create_message(channel_id, user_data.user_id, payload)
        .await?;

    Ok(Json(true))
}
pub async fn create_direct_message(
    State(state): State<AppState>,
    Path(recipient_id): Path<i32>,
    AuthBearer(token): AuthBearer,
    Json(payload): Json<CreateMessageBody>,
) -> Result<Json<bool>, AppError> {
    let user_data = get_token_data(&state, &token).await?;
    if user_data.user_id == recipient_id {
        return Err(AppError::MessageService(Status::invalid_argument(
            "Cannot create a direct message with yourself",
        )));
    }
    let user_exists = state.accounts.user_exists(&recipient_id).await?;
    if !user_exists.found {
        return Err(AppError::MessageService(Status::not_found(
            "User with that id does not exist",
        )));
    }

    state
        .messages
        .create_direct_message(user_data.user_id, recipient_id, payload)
        .await?;

    Ok(Json(true))
}

pub async fn get_messages(
    State(state): State<AppState>,
    Path(channel_id): Path<i32>,
    Query(query): Query<GetMessagesQuery>,
    AuthBearer(token): AuthBearer,
) -> Result<Json<Vec<MessageItemResponse>>, AppError> {
    let user_data = get_token_data(&state, &token).await?;

    let result = state
        .messages
        .clone()
        .get_history(
            HistoryRequest {
                channel_id,
                limit: query.limit.unwrap_or(50),
                page: query.page.unwrap_or(1),
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
pub async fn get_direct_messages(
    State(state): State<AppState>,
    Path(recipient_id): Path<i32>,
    Query(query): Query<GetMessagesQuery>,
    AuthBearer(token): AuthBearer,
) -> Result<Json<Vec<MessageItemResponse>>, AppError> {
    let user_data = get_token_data(&state, &token).await?;

    if user_data.user_id == recipient_id {
        return Err(AppError::MessageService(Status::invalid_argument(
            "Cannot get a direct message with yourself",
        )));
    };
    let user_exists = state.accounts.user_exists(&recipient_id).await?;
    if !user_exists.found {
        return Err(AppError::MessageService(Status::not_found(
            "User with that id does not exist",
        )));
    };
    let result = state
        .messages
        .clone()
        .get_direct_history(
            DirectHistoryRequest {
                recipient_id,
                limit: query.limit.unwrap_or(50),
                page: query.page.unwrap_or(1),
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
