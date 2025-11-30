use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum AppError {
    UserNotFound,
    UserAlreadyExists,
    InternalServerError,
    ValidationError(String),
    NotFound,
    Unauthorized,
    Forbidden,
    Unknown(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = match self {
            AppError::UserNotFound => "User not found".to_string(),
            AppError::UserAlreadyExists => "User already exists".to_string(),
            AppError::InternalServerError => "Internal server error".to_string(),
            AppError::ValidationError(msg) => format!("Validation error: {}", msg),
            AppError::NotFound => "Not found".to_string(),
            AppError::Unauthorized => "Unauthorized".to_string(),
            AppError::Forbidden => "Forbidden".to_string(),
            AppError::Unknown(msg) => format!("Unknown error, all I got is this: {}", msg),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
