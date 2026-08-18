use axum::{Router, routing::get};

use super::handlers::get_messages;
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/messages/{channelId}", get(get_messages))
}
