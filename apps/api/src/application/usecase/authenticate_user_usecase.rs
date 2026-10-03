use std::sync::Arc;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::usecase::errors::UsecaseError,
    domain::{model::user::User, repository::user_repository::UserRepository},
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct AuthenticateUserInput {
    pub sub: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthenticateUserOutput {
    pub user: User,
}

pub struct AuthenticateUserUsecase {
    user_repository: Arc<dyn UserRepository>,
}

impl AuthenticateUserUsecase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        input: AuthenticateUserInput,
    ) -> Result<AuthenticateUserOutput, UsecaseError> {
        let user = self.user_repository.get_user_by_sub(&input.sub).await?;
        Ok(AuthenticateUserOutput { user })
    }
}
