use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::post};

use crate::{
    application::usecase::confirm_sign_up_usecase::ConfirmSignUpInput,
    presentation::{
        handler::confirm_sign_up_handler::ConfirmSignUpHandler,
        middleware::json_validator::JsonValidator,
    },
};

pub struct ConfirmSignUpRouter {
    pub confirm_sign_up: Arc<ConfirmSignUpHandler>,
}

impl ConfirmSignUpRouter {
    pub fn new(confirm_sign_up: Arc<ConfirmSignUpHandler>) -> Self {
        Self { confirm_sign_up }
    }

    pub fn routes(&self) -> Router {
        Router::new().route(
            "/confirm_sign_up",
            post({
                let handler = self.confirm_sign_up.clone();
                move |input: JsonValidator<ConfirmSignUpInput>| {
                    let handler = handler.clone();
                    async move { handler.handle(input).await.into_response() }
                }
            }),
        )
    }
}
