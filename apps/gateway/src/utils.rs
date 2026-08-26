use tonic::{Request, Status, metadata::MetadataValue};

use crate::{
    accounts::{AuthService, ValidateTokenBody},
    app_error::AppError,
    pb::auth::ValidateTokenResponse,
};

pub async fn get_token_data(
    auth_service: AuthService,
    token: String,
) -> Result<ValidateTokenResponse, AppError> {
    if token.trim() == "" {
        return Err(AppError::MessageService(Status::unauthenticated(
            "You are not allowed to view messages of that channel",
        )));
    }
    let user_data = auth_service
        .validate_token(ValidateTokenBody { token })
        .await?;
    if !user_data.is_valid {
        return Err(AppError::MessageService(Status::unauthenticated(
            "Your token expired. Please log-in again",
        )));
    }
    return Ok(user_data);
}

pub fn add_user_id_to_request<T>(mut request: Request<T>, user_id: &i32) -> Request<T> {
    let user_id_val = MetadataValue::from(*user_id);

    request.metadata_mut().insert("x-user-id", user_id_val);
    return request;
}
