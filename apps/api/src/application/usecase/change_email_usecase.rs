use std::sync::Arc;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::{service::identity_provider::IdentityProvider, usecase::errors::UsecaseError},
    domain::model::email::Email,
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ChangeEmailInput {
    pub access_token: String,
    pub email: Email,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEmailOutput {}

pub struct ChangeEmailUsecase {
    identity_provider: Arc<dyn IdentityProvider>,
}

impl ChangeEmailUsecase {
    pub fn new(identity_provider: Arc<dyn IdentityProvider>) -> Self {
        Self { identity_provider }
    }

    pub async fn execute(
        &self,
        input: ChangeEmailInput,
    ) -> Result<ChangeEmailOutput, UsecaseError> {
        self.identity_provider
            .change_email(input.access_token, input.email)
            .await
            .map(|_| ChangeEmailOutput {})
            .map_err(|e| e.into())
    }
}
