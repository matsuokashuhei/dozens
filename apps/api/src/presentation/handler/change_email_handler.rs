use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::usecase::change_email_usecase::{
    ChangeEmailBody, ChangeEmailInput, ChangeEmailUsecase,
};
use crate::presentation::errors::PresentationError;
use crate::presentation::middleware::json_validator::JsonValidator;
use crate::presentation::middleware::user_authenticator::UserAuthenticator;

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
        user_authenticator: UserAuthenticator,
        JsonValidator(body): JsonValidator<ChangeEmailBody>,
    ) -> Result<Response, PresentationError> {
        let input = ChangeEmailInput {
            access_token: user_authenticator.access_token,
            email: body.email,
        };
        self.change_email_usecase.execute(input).await?;
        Ok((StatusCode::NO_CONTENT).into_response())
    }
}
