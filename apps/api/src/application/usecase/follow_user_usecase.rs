use std::sync::Arc;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::usecase::errors::UsecaseError,
    domain::{
        model::{
            user::UserId,
            user_relationship::{UserRelationship, UserRelationshipError},
        },
        repository::{
            errors::RepositoryError, user_relationship_repository::UserRelationshipRepository,
        },
    },
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct FollowUserBody {
    pub followee_id: UserId,
}

#[derive(Debug, Clone)]
pub struct FollowUserInput {
    pub follower_id: UserId,
    pub followee_id: UserId,
}

#[derive(Debug, Clone, Serialize)]
pub struct FollowUserOutput {}

pub struct FollowUserUsecase {
    user_relationship_repository: Arc<dyn UserRelationshipRepository>,
}

impl FollowUserUsecase {
    pub fn new(user_relationship_repository: Arc<dyn UserRelationshipRepository>) -> Self {
        Self {
            user_relationship_repository,
        }
    }

    pub async fn execute(&self, input: FollowUserInput) -> Result<FollowUserOutput, UsecaseError> {
        let user_relationship = UserRelationship::new(input.follower_id, input.followee_id)
            .map_err(|error| match error {
                UserRelationshipError::SelfFollowNotAllowed => UsecaseError::SelfFollowNotAllowed,
            })?;
        match self
            .user_relationship_repository
            .create_user_relationship(user_relationship)
            .await
        {
            // Following an already-followed user is idempotent.
            Ok(_) | Err(RepositoryError::UniqueViolation) => Ok(FollowUserOutput {}),
            Err(error) => Err(error.into()),
        }
    }
}
