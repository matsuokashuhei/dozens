use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::usecase::change_email_usecase::{ChangeEmailInput, ChangeEmailUsecase};
use crate::presentation::errors::PresentationError;
use crate::presentation::middleware::json_validator::JsonValidator;

pub struct ChangeEmailHandler {
    change_email_usecase: Arc<ChangeEmailUsecase>,
}

impl ChangeEmailHandler {
    pub fn new(change_email_usecase: Arc<ChangeEmailUsecase>) -> Self {
        Self {
            change_email_usecase,
        }
    }

    pub async fn handle(
        &self,
        JsonValidator(input): JsonValidator<ChangeEmailInput>,
    ) -> Result<Response, PresentationError> {
        self.change_email_usecase.execute(input).await?;
        Ok((StatusCode::NO_CONTENT).into_response())
    }
}
