use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use crate::application::usecase::errors::UsecaseError;

#[derive(Debug, Error)]
pub enum PresentationError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("internal server error: {0}")]
    InternalServerError(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: String,
}

impl From<UsecaseError> for PresentationError {
    fn from(error: UsecaseError) -> Self {
        match error {
            UsecaseError::NotAuthorized => PresentationError::Unauthorized,
            UsecaseError::InternalError(message) => PresentationError::InternalServerError(message),
            _ => PresentationError::BadRequest(error.to_string()),
        }
    }
}
impl IntoResponse for PresentationError {
    fn into_response(self) -> Response {
        match self {
            PresentationError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    code: "bad_request",
                    message,
                }),
            )
                .into_response(),
            PresentationError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorBody {
                    code: "unauthorized",
                    message: "unauthorized".to_string(),
                }),
            )
                .into_response(),
            PresentationError::InternalServerError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody {
                    code: "internal_server_error",
                    message,
                }),
            )
                .into_response(),
        }
    }
}
