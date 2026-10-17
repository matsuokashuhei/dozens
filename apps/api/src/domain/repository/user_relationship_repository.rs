use async_trait::async_trait;

use crate::domain::{
    model::{user_id::UserId, user_relationship::UserRelationship},
    repository::errors::RepositoryError,
};

#[async_trait]
pub trait UserRelationshipRepository: Send + Sync {
    async fn create_user_relationship(
        &self,
        user_relationship: UserRelationship,
    ) -> Result<UserRelationship, RepositoryError>;

    async fn get_user_relationship(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<UserRelationship, RepositoryError>;

    async fn list_user_relationships_by_follower(
        &self,
        follower_id: UserId,
    ) -> Result<Vec<UserRelationship>, RepositoryError>;

    async fn list_user_relationships_by_followee(
        &self,
        followee_id: UserId,
    ) -> Result<Vec<UserRelationship>, RepositoryError>;

    async fn delete_user_relationship(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<(), RepositoryError>;
}
