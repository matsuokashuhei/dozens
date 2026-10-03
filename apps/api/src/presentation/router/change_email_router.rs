use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::post};

use crate::{
    application::usecase::change_email_usecase::ChangeEmailInput,
    presentation::{
        handler::change_email_handler::ChangeEmailHandler,
        middleware::json_validator::JsonValidator,
    },
};

pub struct ChangeEmailRouter {
    pub change_email: Arc<ChangeEmailHandler>,
}

impl ChangeEmailRouter {
    pub fn new(change_email: Arc<ChangeEmailHandler>) -> Self {
        Self { change_email }
    }

    pub fn routes(&self) -> Router {
        Router::new().route(
            "/change_email",
            post({
                let handler = self.change_email.clone();
                move |input: JsonValidator<ChangeEmailInput>| {
                    let handler = handler.clone();
                    async move { handler.handle(input).await.into_response() }
                }
            }),
        )
    }
}
