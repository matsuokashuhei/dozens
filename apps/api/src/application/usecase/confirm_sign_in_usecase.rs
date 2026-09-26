use std::sync::Arc;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::{
        service::identity_provider::{ConfirmSignInResult, IdentityProvider},
        usecase::errors::UsecaseError,
    },
    domain::model::email::Email,
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ConfirmSignInInput {
    pub session: String,
    pub email: Email,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmSignInOutput {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
}

impl From<ConfirmSignInResult> for ConfirmSignInOutput {
    fn from(result: ConfirmSignInResult) -> Self {
        ConfirmSignInOutput {
            access_token: result.access_token,
            refresh_token: result.refresh_token,
            id_token: result.id_token,
        }
    }
}

pub struct ConfirmSignInUsecase {
    identity_provider: Arc<dyn IdentityProvider>,
}

impl ConfirmSignInUsecase {
    pub fn new(identity_provider: Arc<dyn IdentityProvider>) -> Self {
        Self { identity_provider }
    }

    pub async fn execute(
        &self,
        input: ConfirmSignInInput,
    ) -> Result<ConfirmSignInOutput, UsecaseError> {
        self.identity_provider
            .confirm_sign_in(input.session, input.email, input.code)
            .await
            .map(ConfirmSignInOutput::from)
            .map_err(UsecaseError::from)
    }
}
