use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Password hash error: {0}")]
    PasswordHash(#[from] argon2::password_hash::Error),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::Authentication(ref message) => {
                tracing::warn!("Authentication error: {}", message);
                (StatusCode::UNAUTHORIZED, message.as_str())
            }
            AppError::Authorization(ref message) => {
                tracing::warn!("Authorization error: {}", message);
                (StatusCode::FORBIDDEN, message.as_str())
            }
            AppError::Validation(ref errors) => {
                tracing::warn!("Validation error: {:?}", errors);
                (StatusCode::BAD_REQUEST, "Validation failed")
            }
            AppError::NotFound(ref message) => {
                tracing::warn!("Not found: {}", message);
                (StatusCode::NOT_FOUND, message.as_str())
            }
            AppError::Conflict(ref message) => {
                tracing::warn!("Conflict: {}", message);
                (StatusCode::CONFLICT, message.as_str())
            }
            AppError::BadRequest(ref message) => {
                tracing::warn!("Bad request: {}", message);
                (StatusCode::BAD_REQUEST, message.as_str())
            }
            AppError::Internal(ref message) => {
                tracing::error!("Internal error: {}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::Jwt(ref err) => {
                tracing::warn!("JWT error: {}", err);
                (StatusCode::UNAUTHORIZED, "Invalid token")
            }
            AppError::PasswordHash(ref err) => {
                tracing::error!("Password hash error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::RateLimit(ref message) => {
                tracing::warn!("Rate limit exceeded: {}", message);
                (StatusCode::TOO_MANY_REQUESTS, message.as_str())
            }
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "type": error_type_string(&self)
            }
        }));

        (status, body).into_response()
    }
}

fn error_type_string(error: &AppError) -> &'static str {
    match error {
        AppError::Database(_) => "database_error",
        AppError::Authentication(_) => "authentication_error",
        AppError::Authorization(_) => "authorization_error", 
        AppError::Validation(_) => "validation_error",
        AppError::NotFound(_) => "not_found",
        AppError::Conflict(_) => "conflict",
        AppError::BadRequest(_) => "bad_request",
        AppError::Internal(_) => "internal_error",
        AppError::Jwt(_) => "jwt_error",
        AppError::PasswordHash(_) => "password_hash_error",
        AppError::RateLimit(_) => "rate_limit_error",
    }
}

pub type Result<T> = std::result::Result<T, AppError>;