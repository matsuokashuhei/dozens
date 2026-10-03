use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::delete};

use crate::presentation::{
    handler::sign_out_handler::SignOutHandler, middleware::user_authenticator::UserAuthenticator,
};

pub struct SignOutRouter {
    pub sign_out: Arc<SignOutHandler>,
}

impl SignOutRouter {
    pub fn new(sign_out: Arc<SignOutHandler>) -> Self {
        Self { sign_out }
    }

    pub fn routes(&self) -> Router {
        Router::new().route(
            "/sign_out",
            delete({
                let handler = self.sign_out.clone();
                move |user_authenticator: UserAuthenticator| {
                    let handler = handler.clone();
                    async move { handler.handle(user_authenticator).await.into_response() }
                }
            }),
        )
    }
}
