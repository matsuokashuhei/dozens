use std::sync::Arc;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    application::usecase::create_user_usecase::{CreateUserInput, CreateUserUsecase},
    presentation::{errors::PresentationError, middleware::json_validator::JsonValidator},
};

pub struct CreateUserHandler {
    create_user_usecase: Arc<CreateUserUsecase>,
}

impl CreateUserHandler {
    pub fn new(create_user_usecase: Arc<CreateUserUsecase>) -> Self {
        Self {
            create_user_usecase,
        }
    }

    pub async fn handle(
        &self,
        JsonValidator(input): JsonValidator<CreateUserInput>,
    ) -> Result<Response, PresentationError> {
        let user = self.create_user_usecase.execute(input).await?;
        Ok((StatusCode::CREATED, Json(user)).into_response())
    }
}
