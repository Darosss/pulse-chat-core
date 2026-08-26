use axum::{
    extract::{Query, State, WebSocketUpgrade},
    response::Response,
};
use serde::Deserialize;

use crate::{app_error::AppError, app_state::AppState, utils::get_token_data};

#[derive(Deserialize)]
pub struct WsAuthQuery {
    pub token: String,
}

pub async fn handle_socket(
    State(state): State<AppState>,
    Query(query): Query<WsAuthQuery>,
    ws: WebSocketUpgrade,
) -> Result<Response, AppError> {
    let accounts = state.accounts.clone();
    let user_data: crate::pb::auth::ValidateTokenResponse =
        get_token_data(accounts, query.token).await?;

    Ok(ws.on_upgrade(move |socket| {
        let messages = state.messages.clone();
        let redis = state.redis.clone();
        let redis_client = state.redis_client.clone();
        let rooms = state.rooms.clone();
        state.ws_state.handle_socket(
            socket,
            redis,
            redis_client,
            rooms,
            messages,
            user_data.user_id,
        )
    }))
}
