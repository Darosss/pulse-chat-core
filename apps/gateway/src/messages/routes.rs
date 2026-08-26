use axum::{
    Router,
    routing::{get, post},
};

use super::handlers::{create_message, get_messages};
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/messages/{channelId}", get(get_messages))
        .route("/messages/{channelId}", post(create_message))
}
