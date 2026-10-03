use std::sync::Arc;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::usecase::errors::UsecaseError,
    domain::{model::user::User, repository::user_repository::UserRepository},
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct GetUserInput {
    pub sub: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUserOutput {
    pub user: User,
}

pub struct GetUserUsecase {
    user_repository: Arc<dyn UserRepository>,
}

impl GetUserUsecase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: GetUserInput) -> Result<GetUserOutput, UsecaseError> {
        let user = self.user_repository.get_user_by_sub(&input.sub).await?;
        Ok(GetUserOutput { user })
    }
}
