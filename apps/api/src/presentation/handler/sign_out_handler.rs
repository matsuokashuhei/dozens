use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::usecase::sign_out_usecase::{SignOutInput, SignOutUsecase};
use crate::presentation::errors::PresentationError;
use crate::presentation::middleware::json_validator::JsonValidator;

pub struct SignOutHandler {
    sign_out_usecase: Arc<SignOutUsecase>,
}

impl SignOutHandler {
    pub fn new(sign_out_usecase: Arc<SignOutUsecase>) -> Self {
        Self { sign_out_usecase }
    }

    pub async fn handle(
        &self,
        JsonValidator(input): JsonValidator<SignOutInput>,
    ) -> Result<Response, PresentationError> {
        self.sign_out_usecase.execute(input).await?;
        Ok((StatusCode::NO_CONTENT).into_response())
    }
}
