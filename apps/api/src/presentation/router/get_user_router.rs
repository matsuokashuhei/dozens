use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::get};

use crate::presentation::{
    handler::get_user_handler::GetUserHandler, middleware::user_authenticator::UserAuthenticator,
};

pub struct GetUserRouter {
    pub get_user: Arc<GetUserHandler>,
}

impl GetUserRouter {
    pub fn new(get_user: Arc<GetUserHandler>) -> Self {
        Self { get_user }
    }

    pub fn routes(&self) -> Router {
        Router::new().route(
            "/user",
            get({
                let handler = self.get_user.clone();
                move |user: UserAuthenticator| {
                    let handler = handler.clone();
                    async move { handler.handle(user).await.into_response() }
                }
            }),
        )
    }
}
