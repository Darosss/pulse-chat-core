use tonic::{Request, Status, metadata::MetadataValue};

use crate::{
    accounts::token_utils::{TokenClaims, validate_request_jwt},
    app_error::AppError,
    app_state::AppState,
};

pub async fn get_token_data(state: &AppState, token: &str) -> Result<TokenClaims, AppError> {
    if token.trim() == "" {
        return Err(AppError::Gateway(Status::unauthenticated(
            "You are unauthenticated",
        )));
    }

    let token_claims = validate_request_jwt(state, &token).await?;

    return Ok(token_claims);
}

pub fn add_user_id_to_request<T>(mut request: Request<T>, user_id: &i32) -> Request<T> {
    let user_id_val = MetadataValue::from(*user_id);

    request.metadata_mut().insert("x-user-id", user_id_val);
    return request;
}
