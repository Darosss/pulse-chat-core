use axum::{Json, extract::State};
use axum_auth::AuthBearer;
use serde::{Deserialize, Serialize};

use crate::{
    app_error::AppError, app_state::AppState, pb::guild::GuildResponse, utils::get_token_data,
};

#[derive(Serialize, Deserialize)]
pub struct CreateGuildBody {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GuildResponseSerialized {
    pub id: String,
    pub name: String,
    pub owner_id: i32,
    pub created_at: String,
}
impl From<GuildResponse> for GuildResponseSerialized {
    fn from(value: GuildResponse) -> Self {
        Self {
            id: value.id,
            name: value.name,
            owner_id: value.owner_id,
            created_at: value.created_at,
        }
    }
}
pub async fn create_guild(
    State(state): State<AppState>,
    AuthBearer(token): AuthBearer,
    Json(payload): Json<CreateGuildBody>,
) -> Result<Json<GuildResponseSerialized>, AppError> {
    let user_data = get_token_data(&state, &token).await?;

    let response = state
        .guilds
        .create_guild(user_data.user_id, payload)
        .await?;

    Ok(Json(response.into()))
}
