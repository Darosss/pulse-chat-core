use axum::{Router, routing::get};

use super::handlers::handle_socket;
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/ws", get(handle_socket))
}
