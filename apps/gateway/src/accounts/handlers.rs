use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use crate::{app_error::AppError, app_state::AppState, pb::auth::AuthResponse};

#[derive(Serialize, Deserialize)]
pub struct AuthResponseSerialized {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}

impl From<AuthResponse> for AuthResponseSerialized {
    fn from(value: AuthResponse) -> Self {
        Self {
            token: value.token,
            user_id: value.user_id,
            username: value.username,
        }
    }
}

#[derive(Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct RegisterBody {
    pub email: String,
    pub password: String,
    pub username: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginBody>,
) -> Result<Json<AuthResponseSerialized>, AppError> {
    let result = state
        .accounts
        .login(LoginBody {
            email: payload.email,
            password: payload.password,
        })
        .await?;

    Ok(Json(result.into()))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterBody>,
) -> Result<Json<AuthResponseSerialized>, AppError> {
    let result = state
        .accounts
        .register(RegisterBody {
            email: payload.email,
            password: payload.password,
            username: payload.username,
        })
        .await?;

    Ok(Json(result.into()))
}
