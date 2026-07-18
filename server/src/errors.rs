use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource conflict: {0}")]
    Conflict(String),

    #[error("Configuration Error")]
    Config(#[from] crate::config::ConfigError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal,

    #[error("Password hashing failed")]
    PasswordHash(#[from] argon2::password_hash::Error),

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Invalid token")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Database(ref e) => {
                tracing::error!("SQLx error: {:#?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHash(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Jwt(_) => StatusCode::UNAUTHORIZED,
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}
