use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tonic::Status;

use crate::{app_error::AppError, app_state::AppState, redis_utils::auth::get_blacklist_key};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub user_id: i32,
    pub username: String,
    pub jti: String,
    pub exp: i64,
}

pub async fn validate_request_jwt(state: &AppState, token: &str) -> Result<TokenClaims, AppError> {
    let decoding_key = state.get_or_fetch_public_key().await?;

    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let token_data = jsonwebtoken::decode::<TokenClaims>(token, &decoding_key, &validation)
        .map_err(|_| {
            AppError::AccountsService(Status::unauthenticated("Invalid or expired token"))
        })?;

    let claims = token_data.claims;

    let mut redis_client = state.redis.clone();

    let is_blacklisted: bool = redis_client
        .exists(get_blacklist_key(&claims.jti))
        .await
        .unwrap_or(false);

    if is_blacklisted {
        return Err(AppError::AccountsService(Status::unauthenticated(
            "Token has been revoked",
        )));
    }

    Ok(claims)
}
