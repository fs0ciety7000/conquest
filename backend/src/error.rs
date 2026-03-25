use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Insufficient resources: {0}")]
    InsufficientResources(String),

    #[error("Requirements not met: {0}")]
    RequirementsNotMet(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Internal(e.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::NotFound(msg)                => (StatusCode::NOT_FOUND,              "NOT_FOUND",              msg.clone()),
            AppError::Unauthorized(msg)            => (StatusCode::UNAUTHORIZED,            "UNAUTHORIZED",           msg.clone()),
            AppError::Forbidden(msg)               => (StatusCode::FORBIDDEN,               "FORBIDDEN",              msg.clone()),
            AppError::BadRequest(msg)              => (StatusCode::BAD_REQUEST,             "BAD_REQUEST",            msg.clone()),
            AppError::Conflict(msg)                => (StatusCode::CONFLICT,                "CONFLICT",               msg.clone()),
            AppError::InsufficientResources(msg)   => (StatusCode::UNPROCESSABLE_ENTITY,    "INSUFFICIENT_RESOURCES", msg.clone()),
            AppError::RequirementsNotMet(msg)      => (StatusCode::UNPROCESSABLE_ENTITY,    "REQUIREMENTS_NOT_MET",   msg.clone()),
            AppError::Database(e) => {
                tracing::error!(error = %e, "Database error");
                (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", "Internal database error".to_string())
            }
            AppError::Internal(msg) => {
                tracing::error!(error = %msg, "Internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "Internal server error".to_string())
            }
        };

        (status, Json(json!({ "error": { "code": code, "message": message } }))).into_response()
    }
}
