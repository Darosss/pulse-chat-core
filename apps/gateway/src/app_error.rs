use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tonic::{
    Code::{AlreadyExists, NotFound, PermissionDenied, Unauthenticated, Unavailable},
    Status,
};

pub enum AppError {
    MessageService(Status),
    AccountsService(Status),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::MessageService(status) | AppError::AccountsService(status) => {
                let status_code = match status.code() {
                    Unavailable => StatusCode::SERVICE_UNAVAILABLE,
                    AlreadyExists => StatusCode::CONFLICT,
                    NotFound => StatusCode::NOT_FOUND,
                    Unauthenticated => StatusCode::UNAUTHORIZED,
                    PermissionDenied => StatusCode::FORBIDDEN,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                status_code.into_response()
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
