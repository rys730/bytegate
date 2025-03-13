use core::error;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::Serialize;
use thiserror::Error;
use sqlx::Error as SqlxError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Unexpected Error: {0}")]
    UnexpectedError(String),
}

pub type Result<T> = anyhow::Result<T, ServiceError>;

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    error_code: String,
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        let (status, message, error_code) = match self {
            ServiceError::DatabaseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error", "BTG_5001")
            },
            ServiceError::UnexpectedError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error", "BTG_5000")
            }
        };

        let response = Json(ErrorResponse {
            message: message.to_string(),
            error_code: error_code.to_string(),
        });
        (status, response).into_response()
    }
}