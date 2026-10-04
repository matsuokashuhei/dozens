use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::post};

use crate::{
    application::usecase::confirm_sign_in_usecase::ConfirmSignInInput,
    presentation::{
        handler::confirm_sign_in_handler::ConfirmSignInHandler,
        middleware::json_validator::JsonValidator,
    },
};

pub struct ConfirmSignInRouter {
    pub confirm_sign_in: Arc<ConfirmSignInHandler>,
}

impl ConfirmSignInRouter {
    pub fn new(confirm_sign_in: Arc<ConfirmSignInHandler>) -> Self {
        Self { confirm_sign_in }
    }

    pub fn routes(&self) -> Router {
        Router::new().route(
            "/confirm_sign_in",
            post({
                let handler = self.confirm_sign_in.clone();
                move |input: JsonValidator<ConfirmSignInInput>| {
                    let handler = handler.clone();
                    async move { handler.handle(input).await.into_response() }
                }
            }),
        )
    }
}
