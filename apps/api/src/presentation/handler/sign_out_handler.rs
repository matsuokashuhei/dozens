use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::usecase::sign_out_usecase::{SignOutInput, SignOutUsecase};
use crate::presentation::errors::PresentationError;
use crate::presentation::middleware::user_authenticator::UserAuthenticator;

pub struct SignOutHandler {
    sign_out_usecase: Arc<SignOutUsecase>,
}

impl SignOutHandler {
    pub fn new(sign_out_usecase: Arc<SignOutUsecase>) -> Self {
        Self { sign_out_usecase }
    }

    pub async fn handle(
        &self,
        user_authenticator: UserAuthenticator,
    ) -> Result<Response, PresentationError> {
        self.sign_out_usecase
            .execute(SignOutInput {
                access_token: user_authenticator.access_token,
            })
            .await?;
        Ok((StatusCode::NO_CONTENT).into_response())
    }
}
