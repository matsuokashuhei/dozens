use std::sync::Arc;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::{
        service::identity_provider::{IdentityProvider, SignUpResult},
        usecase::errors::UsecaseError,
    },
    domain::{
        model::{email::Email, user::User, user_identity::UserIdentity, username::Username},
        repository::{
            user_identity_repository::UserIdentityRepository, user_repository::UserRepository,
        },
    },
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct SignUpInput {
    pub email: Email,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpOutput {
    pub email: Email,
}

pub struct SignUpUsecase {
    identity_provider: Arc<dyn IdentityProvider>,
    user_repository: Arc<dyn UserRepository>,
    user_identity_repository: Arc<dyn UserIdentityRepository>,
}

impl SignUpUsecase {
    pub fn new(
        identity_provider: Arc<dyn IdentityProvider>,
        user_repository: Arc<dyn UserRepository>,
        user_identity_repository: Arc<dyn UserIdentityRepository>,
    ) -> Self {
        Self {
            identity_provider,
            user_repository,
            user_identity_repository,
        }
    }

    pub async fn execute(&self, input: SignUpInput) -> Result<SignUpOutput, UsecaseError> {
        let result = self.identity_provider.sign_up(input.email.clone()).await;
        match result {
            Ok(sign_up_result) => {
                self.create_user(sign_up_result).await?;
                Ok(SignUpOutput {
                    email: input.email.clone(),
                })
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn create_user(&self, sign_up_result: SignUpResult) -> Result<User, UsecaseError> {
        let user = User::new(Username::generate().as_str().to_string());
        let user = self.user_repository.create_user(user).await?;
        let user_identity = UserIdentity::new(user.id, sign_up_result.iss, sign_up_result.sub);
        self.user_identity_repository
            .create_user_identity(user_identity)
            .await
            .map_err(UsecaseError::from)?;
        Ok(user)
    }
}
