use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::usecase::follow_user_usecase::{
    FollowUserBody, FollowUserInput, FollowUserUsecase,
};
use crate::presentation::errors::PresentationError;
use crate::presentation::middleware::json_validator::JsonValidator;
use crate::presentation::middleware::user_authenticator::UserAuthenticator;

pub struct FollowUserHandler {
    follow_user_usecase: Arc<FollowUserUsecase>,
}

impl FollowUserHandler {
    pub fn new(follow_user_usecase: Arc<FollowUserUsecase>) -> Self {
        Self {
            follow_user_usecase,
        }
    }

    pub async fn handle(
        &self,
        user_authenticator: UserAuthenticator,
        JsonValidator(body): JsonValidator<FollowUserBody>,
    ) -> Result<Response, PresentationError> {
        let input = FollowUserInput {
            follower_id: user_authenticator.user.id,
            followee_id: body.followee_id,
        };
        self.follow_user_usecase.execute(input).await?;
        Ok((StatusCode::NO_CONTENT).into_response())
    }
}
