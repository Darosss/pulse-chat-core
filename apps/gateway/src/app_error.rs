use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tonic::{
    Code::{NotFound, PermissionDenied, Unauthenticated},
    Status,
};

pub enum AppError {
    MessageService(Status),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::MessageService(status) => {
                let status_code = match status.code() {
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
        Self::MessageService(status)
    }
}
