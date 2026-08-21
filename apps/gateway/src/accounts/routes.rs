use axum::{Router, routing::post};

use super::handlers::{login, register, validate_token};
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/validate-token", post(validate_token))
}
