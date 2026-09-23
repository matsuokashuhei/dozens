use std::sync::Arc;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::usecase::errors::UsecaseError,
    domain::{model::user::User, repository::user_repository::UserRepository},
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUserInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserOutput {
    pub id: String,
    pub name: String,
}

impl From<User> for CreateUserOutput {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name,
        }
    }
}

pub struct CreateUserUsecase {
    #[allow(dead_code)]
    user_repository: Arc<dyn UserRepository>,
}

impl CreateUserUsecase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: CreateUserInput) -> Result<CreateUserOutput, UsecaseError> {
        let user = User::new(input.name);
        self.user_repository
            .create_user(user)
            .await
            .map(User::into)
            .map_err(UsecaseError::from)
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, faker::name::raw::Name, locales::EN};
    use uuid::Uuid;

    use crate::infrastructure::repository::{
        build_db_connection, user_repository::UserRepositoryImpl,
    };

    use super::*;

    #[tokio::test]
    async fn test_create_user_usecase() {
        let db = build_db_connection().await.unwrap();
        let user_repository = UserRepositoryImpl::new(db);
        let usecase = CreateUserUsecase::new(Arc::new(user_repository));
        let input = CreateUserInput {
            name: Name(EN).fake(),
        };
        let output = usecase.execute(input.clone()).await.unwrap();
        assert!(Uuid::parse_str(&output.id).is_ok());
        assert_eq!(output.name, input.name);
    }
}
