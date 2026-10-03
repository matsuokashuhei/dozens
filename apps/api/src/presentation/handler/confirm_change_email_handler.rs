use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::usecase::confirm_change_email_usecase::{
    ConfirmChangeEmailInput, ConfirmChangeEmailUsecase,
};
use crate::presentation::errors::PresentationError;
use crate::presentation::middleware::json_validator::JsonValidator;

pub struct ConfirmChangeEmailHandler {
    confirm_change_email_usecase: Arc<ConfirmChangeEmailUsecase>,
}

impl ConfirmChangeEmailHandler {
    pub fn new(confirm_change_email_usecase: Arc<ConfirmChangeEmailUsecase>) -> Self {
        Self {
            confirm_change_email_usecase,
        }
    }

    pub async fn handle(
        &self,
        JsonValidator(input): JsonValidator<ConfirmChangeEmailInput>,
    ) -> Result<Response, PresentationError> {
        self.confirm_change_email_usecase.execute(input).await?;
        Ok((StatusCode::NO_CONTENT).into_response())
    }
}
