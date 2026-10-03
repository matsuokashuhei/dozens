use std::sync::Arc;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::usecase::confirm_sign_in_usecase::{
    ConfirmSignInInput, ConfirmSignInUsecase,
};
use crate::presentation::errors::PresentationError;
use crate::presentation::middleware::json_validator::JsonValidator;

pub struct ConfirmSignInHandler {
    confirm_sign_in_usecase: Arc<ConfirmSignInUsecase>,
}

impl ConfirmSignInHandler {
    pub fn new(confirm_sign_in_usecase: Arc<ConfirmSignInUsecase>) -> Self {
        Self {
            confirm_sign_in_usecase,
        }
    }

    pub async fn handle(
        &self,
        JsonValidator(input): JsonValidator<ConfirmSignInInput>,
    ) -> Result<Response, PresentationError> {
        let result = self.confirm_sign_in_usecase.execute(input).await?;
        Ok((StatusCode::OK, Json(result)).into_response())
    }
}
