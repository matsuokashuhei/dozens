use std::sync::Arc;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::usecase::unfollow_user_usecase::{UnfollowUserInput, UnfollowUserUsecase};
use crate::domain::model::user::UserId;
use crate::presentation::errors::PresentationError;
use crate::presentation::middleware::user_authenticator::UserAuthenticator;

pub struct UnfollowUserHandler {
    unfollow_user_usecase: Arc<UnfollowUserUsecase>,
}

impl UnfollowUserHandler {
    pub fn new(unfollow_user_usecase: Arc<UnfollowUserUsecase>) -> Self {
        Self {
            unfollow_user_usecase,
        }
    }

    pub async fn handle(
        &self,
        user_authenticator: UserAuthenticator,
        Path(followee_id): Path<UserId>,
    ) -> Result<Response, PresentationError> {
        let input = UnfollowUserInput {
            follower_id: user_authenticator.user.id,
            followee_id,
        };
        self.unfollow_user_usecase.execute(input).await?;
        Ok((StatusCode::NO_CONTENT).into_response())
    }
}
