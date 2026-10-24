use std::sync::Arc;

use serde::Serialize;

use crate::{
    application::usecase::errors::UsecaseError,
    domain::{
        model::user::UserId, repository::user_relationship_repository::UserRelationshipRepository,
    },
};

#[derive(Debug, Clone)]
pub struct UnfollowUserInput {
    pub follower_id: UserId,
    pub followee_id: UserId,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnfollowUserOutput {}

pub struct UnfollowUserUsecase {
    user_relationship_repository: Arc<dyn UserRelationshipRepository>,
}

impl UnfollowUserUsecase {
    pub fn new(user_relationship_repository: Arc<dyn UserRelationshipRepository>) -> Self {
        Self {
            user_relationship_repository,
        }
    }

    pub async fn execute(
        &self,
        input: UnfollowUserInput,
    ) -> Result<UnfollowUserOutput, UsecaseError> {
        // Deleting a relationship that does not exist is idempotent.
        self.user_relationship_repository
            .delete_user_relationship(input.follower_id, input.followee_id)
            .await
            .map(|_| UnfollowUserOutput {})
            .map_err(|error| error.into())
    }
}
