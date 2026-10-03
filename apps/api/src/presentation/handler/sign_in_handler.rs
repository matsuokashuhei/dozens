use std::sync::Arc;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::usecase::sign_in_usecase::{SignInInput, SignInUsecase};
use crate::presentation::errors::PresentationError;
use crate::presentation::middleware::json_validator::JsonValidator;

pub struct SignInHandler {
    sign_in_usecase: Arc<SignInUsecase>,
}

impl SignInHandler {
    pub fn new(sign_in_usecase: Arc<SignInUsecase>) -> Self {
        Self { sign_in_usecase }
    }

    pub async fn handle(
        &self,
        JsonValidator(input): JsonValidator<SignInInput>,
    ) -> Result<Response, PresentationError> {
        let result = self.sign_in_usecase.execute(input).await?;
        Ok((StatusCode::OK, Json(result)).into_response())
    }
}
