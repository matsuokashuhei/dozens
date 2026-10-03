use std::sync::Arc;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::application::{
    service::identity_provider::IdentityProvider, usecase::errors::UsecaseError,
};

#[derive(Debug, Clone)]
pub struct ConfirmChangeEmailInput {
    pub access_token: String,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ConfirmChangeEmailBody {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmChangeEmailOutput {}

pub struct ConfirmChangeEmailUsecase {
    identity_provider: Arc<dyn IdentityProvider>,
}

impl ConfirmChangeEmailUsecase {
    pub fn new(identity_provider: Arc<dyn IdentityProvider>) -> Self {
        Self { identity_provider }
    }

    pub async fn execute(
        &self,
        input: ConfirmChangeEmailInput,
    ) -> Result<ConfirmChangeEmailOutput, UsecaseError> {
        self.identity_provider
            .confirm_change_email(input.access_token, input.code)
            .await
            .map(|_| ConfirmChangeEmailOutput {})
            .map_err(|e| e.into())
    }
}
