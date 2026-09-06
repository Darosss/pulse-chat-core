use axum::{Router, routing::post};

use super::handlers::create_guild;
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/guilds", post(create_guild))
}
