use std::sync::Arc;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::{
        service::identity_provider::{ConfirmSignUpResult, IdentityProvider},
        usecase::errors::UsecaseError,
    },
    domain::model::email::Email,
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ConfirmSignUpInput {
    pub email: Email,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmSignUpOutput {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
}

impl From<ConfirmSignUpResult> for ConfirmSignUpOutput {
    fn from(result: ConfirmSignUpResult) -> Self {
        ConfirmSignUpOutput {
            access_token: result.access_token,
            refresh_token: result.refresh_token,
            id_token: result.id_token,
        }
    }
}

pub struct ConfirmSignUpUsecase {
    identity_provider: Arc<dyn IdentityProvider>,
}

impl ConfirmSignUpUsecase {
    pub fn new(identity_provider: Arc<dyn IdentityProvider>) -> Self {
        Self { identity_provider }
    }

    pub async fn execute(
        &self,
        input: ConfirmSignUpInput,
    ) -> Result<ConfirmSignUpOutput, UsecaseError> {
        self.identity_provider
            .confirm_sign_up(input.email, input.code)
            .await
            .map(ConfirmSignUpOutput::from)
            .map_err(UsecaseError::from)
    }
}
