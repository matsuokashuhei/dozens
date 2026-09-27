use std::sync::Arc;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    application::usecase::get_user_usecase::{GetUserInput, GetUserUsecase},
    presentation::{
        errors::PresentationError,
        middleware::authenticator::{Authenticator, AuthenticatorError},
    },
};

pub struct GetUserHandler {
    get_user_usecase: Arc<GetUserUsecase>,
}

impl GetUserHandler {
    pub fn new(get_user_usecase: Arc<GetUserUsecase>) -> Self {
        Self { get_user_usecase }
    }

    pub async fn handle(&self, auth: Authenticator) -> Result<Response, PresentationError> {
        let Some(sub) = auth.subject() else {
            return Ok(AuthenticatorError::Unauthorized.into_response());
        };
        let result = self
            .get_user_usecase
            .execute(GetUserInput {
                sub: sub.to_string(),
            })
            .await?;
        Ok((StatusCode::OK, Json(result)).into_response())
    }
}
