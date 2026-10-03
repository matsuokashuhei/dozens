use std::sync::Arc;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::usecase::sign_up_usecase::{SignUpInput, SignUpUsecase};
use crate::presentation::errors::PresentationError;
use crate::presentation::middleware::json_validator::JsonValidator;

pub struct SignUpHandler {
    sign_up_usecase: Arc<SignUpUsecase>,
}

impl SignUpHandler {
    pub fn new(sign_up_usecase: Arc<SignUpUsecase>) -> Self {
        Self { sign_up_usecase }
    }

    pub async fn handle(
        &self,
        JsonValidator(input): JsonValidator<SignUpInput>,
    ) -> Result<Response, PresentationError> {
        let result = self.sign_up_usecase.execute(input).await?;
        Ok((StatusCode::OK, Json(result)).into_response())
    }
}
