use async_trait::async_trait;

use crate::domain::{model::user::User, repository::errors::RepositoryError};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, RepositoryError>;
}
