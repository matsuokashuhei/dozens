use std::sync::Arc;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::application::{
    service::identity_provider::IdentityProvider, usecase::errors::UsecaseError,
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct SignOutInput {
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignOutOutput {}

pub struct SignOutUsecase {
    identity_provider: Arc<dyn IdentityProvider>,
}

impl SignOutUsecase {
    pub fn new(identity_provider: Arc<dyn IdentityProvider>) -> Self {
        Self { identity_provider }
    }

    pub async fn execute(&self, input: SignOutInput) -> Result<SignOutOutput, UsecaseError> {
        self.identity_provider
            .sign_out(input.access_token)
            .await
            .map(|_| SignOutOutput {})
            .map_err(|e| e.into())
    }
}
