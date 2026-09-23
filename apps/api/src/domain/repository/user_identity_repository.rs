use async_trait::async_trait;

use crate::domain::model::user_identity::UserIdentity;
use crate::domain::repository::errors::RepositoryError;

#[async_trait]
pub trait UserIdentityRepository: Send + Sync {
    async fn create_user_identity(
        &self,
        user_identity: UserIdentity,
    ) -> Result<UserIdentity, RepositoryError>;
}
