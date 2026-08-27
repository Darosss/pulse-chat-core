use serde::{Deserialize, Serialize};
use tonic::Status;

use crate::{app_error::AppError, app_state::AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub user_id: i32,
    pub username: String,
    pub jti: String,
    pub exp: usize,
}

pub async fn validate_request_jwt(state: &AppState, token: &str) -> Result<TokenClaims, AppError> {
    let decoding_key = state.get_or_fetch_public_key().await?;

    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let token_data = jsonwebtoken::decode::<TokenClaims>(token, &decoding_key, &validation)
        .map_err(|_| {
            AppError::AccountsService(Status::unauthenticated("Invalid or expired token"))
        })?;

    Ok(token_data.claims)
}
