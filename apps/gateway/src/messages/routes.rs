use axum::{
    Router,
    routing::{get, post},
};

use super::handlers::{chat_socket, create_message, get_messages};
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/messages/{channelId}", get(get_messages))
        .route("/messages/{channelId}", post(create_message))
        .route("/ws/chat/{channelId}", get(chat_socket))
}
