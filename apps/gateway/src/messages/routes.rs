use axum::{
    Router,
    routing::{get, post},
};

use super::handlers::{create_direct_message, create_message, get_direct_messages, get_messages};
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/messages/{channel_id}", get(get_messages))
        .route("/messages/{channel_id}", post(create_message))
        .route("/messages/dm/{recipient_id}", get(get_direct_messages))
        .route("/messages/dm/{recipient_id}", post(create_direct_message))
}
