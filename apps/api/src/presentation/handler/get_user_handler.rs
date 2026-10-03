use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    application::usecase::authenticate_user_usecase::AuthenticateUserOutput,
    presentation::middleware::user_authenticator::UserAuthenticator,
};

#[derive(Default)]
pub struct GetUserHandler;

impl GetUserHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(&self, auth: UserAuthenticator) -> Response {
        (
            StatusCode::OK,
            Json(AuthenticateUserOutput { user: auth.user }),
        )
            .into_response()
    }
}
