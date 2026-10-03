use std::sync::Arc;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::{service::identity_provider::IdentityProvider, usecase::errors::UsecaseError},
    domain::model::email::Email,
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct SignInInput {
    pub email: Email,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInOutput {
    pub email: Email,
    pub session: String,
}

pub struct SignInUsecase {
    identity_provider: Arc<dyn IdentityProvider>,
}

impl SignInUsecase {
    pub fn new(identity_provider: Arc<dyn IdentityProvider>) -> Self {
        Self { identity_provider }
    }

    pub async fn execute(&self, input: SignInInput) -> Result<SignInOutput, UsecaseError> {
        let result = self.identity_provider.sign_in(input.email.clone()).await;
        match result {
            Ok(result) => Ok(SignInOutput {
                email: input.email.clone(),
                session: result.session,
            }),
            Err(e) => Err(e.into()),
        }
    }
}
