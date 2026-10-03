use std::sync::Arc;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::usecase::confirm_sign_up_usecase::{
    ConfirmSignUpInput, ConfirmSignUpUsecase,
};
use crate::presentation::errors::PresentationError;
use crate::presentation::middleware::json_validator::JsonValidator;

pub struct ConfirmSignUpHandler {
    confirm_sign_up_usecase: Arc<ConfirmSignUpUsecase>,
}

impl ConfirmSignUpHandler {
    pub fn new(confirm_sign_up_usecase: Arc<ConfirmSignUpUsecase>) -> Self {
        Self {
            confirm_sign_up_usecase,
        }
    }

    pub async fn handle(
        &self,
        JsonValidator(input): JsonValidator<ConfirmSignUpInput>,
    ) -> Result<Response, PresentationError> {
        let result = self.confirm_sign_up_usecase.execute(input).await?;
        Ok((StatusCode::OK, Json(result)).into_response())
    }
}
