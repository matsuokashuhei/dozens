use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::post};

use crate::{
    application::usecase::confirm_change_email_usecase::ConfirmChangeEmailBody,
    presentation::{
        handler::confirm_change_email_handler::ConfirmChangeEmailHandler,
        middleware::json_validator::JsonValidator,
        middleware::user_authenticator::UserAuthenticator,
    },
};

pub struct ConfirmChangeEmailRouter {
    pub confirm_change_email: Arc<ConfirmChangeEmailHandler>,
}

impl ConfirmChangeEmailRouter {
    pub fn new(confirm_change_email: Arc<ConfirmChangeEmailHandler>) -> Self {
        Self {
            confirm_change_email,
        }
    }

    pub fn routes(&self) -> Router {
        Router::new().route(
            "/confirm_change_email",
            post({
                let handler = self.confirm_change_email.clone();
                move |user_authenticator: UserAuthenticator,
                      input: JsonValidator<ConfirmChangeEmailBody>| {
                    let handler = handler.clone();
                    async move {
                        handler
                            .handle(user_authenticator, input)
                            .await
                            .into_response()
                    }
                }
            }),
        )
    }
}
