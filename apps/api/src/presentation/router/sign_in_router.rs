use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::post};

use crate::{
    application::usecase::sign_in_usecase::SignInInput,
    presentation::{
        handler::sign_in_handler::SignInHandler, middleware::json_validator::JsonValidator,
    },
};

pub struct SignInRouter {
    pub sign_in: Arc<SignInHandler>,
}

impl SignInRouter {
    pub fn new(sign_in: Arc<SignInHandler>) -> Self {
        Self { sign_in }
    }

    pub fn routes(&self) -> Router {
        Router::new().route(
            "/sign_in",
            post({
                let handler = self.sign_in.clone();
                move |input: JsonValidator<SignInInput>| {
                    let handler = handler.clone();
                    async move { handler.handle(input).await.into_response() }
                }
            }),
        )
    }
}
