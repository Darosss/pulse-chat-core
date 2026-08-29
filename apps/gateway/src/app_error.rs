use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tonic::{
    Code::{AlreadyExists, NotFound, PermissionDenied, Unauthenticated, Unavailable},
    Status,
};

#[derive(Debug)]
pub enum AppError {
    Gateway(Status),
    MessageService(Status),
    AccountsService(Status),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::MessageService(status)
            | AppError::AccountsService(status)
            | AppError::Gateway(status) => {
                let status_code = match status.code() {
                    Unavailable => StatusCode::SERVICE_UNAVAILABLE,
                    AlreadyExists => StatusCode::CONFLICT,
                    NotFound => StatusCode::NOT_FOUND,
                    Unauthenticated => StatusCode::UNAUTHORIZED,
                    PermissionDenied => StatusCode::FORBIDDEN,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let error_message = if status.message().is_empty() {
                    "An unexpected error occured".to_string()
                } else {
                    status.message().to_string()
                };
                let body = Json(ErrorResponse {
                    error: error_message,
                    code: format!("{:?}", status.code()),
                });

                (status_code, body).into_response()
            }
        }
    }
}

impl From<Status> for AppError {
    fn from(status: Status) -> Self {
        Self::MessageService(status.clone());
        Self::AccountsService(status.clone())
    }
}
