use axum::{Router, routing::post};

use super::handlers::{login, logout, register};
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/register", post(register))
}
