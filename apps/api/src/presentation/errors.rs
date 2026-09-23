use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::application::usecase::errors::UsecaseError;

#[derive(Debug, Error)]
pub enum PresentationError {
    #[error("bad request: {0}")]
    BadRequest(String),
}

impl From<UsecaseError> for PresentationError {
    fn from(error: UsecaseError) -> Self {
        PresentationError::BadRequest(error.to_string())
    }
}
impl IntoResponse for PresentationError {
    fn into_response(self) -> Response {
        match self {
            PresentationError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, message).into_response()
            }
        }
    }
}
