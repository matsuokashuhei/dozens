use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::post};

use crate::{
    application::usecase::create_user_usecase::CreateUserInput,
    presentation::{
        handler::create_user_handler::CreateUserHandler, middleware::json_validator::JsonValidator,
    },
};

pub struct UserRouter {
    pub create_user: Arc<CreateUserHandler>,
}

impl UserRouter {
    pub fn new(create_user: Arc<CreateUserHandler>) -> Self {
        Self { create_user }
    }

    pub fn routes(&self) -> Router {
        Router::new().route(
            "/users",
            post({
                let handler = self.create_user.clone();
                move |input: JsonValidator<CreateUserInput>| {
                    let handler = handler.clone();
                    async move { handler.handle(input).await.into_response() }
                }
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        Router,
        body::Body,
        http::{HeaderValue, Method, Request, StatusCode, header::CONTENT_TYPE},
    };
    use fake::{Fake, faker::name::raw::Name, locales::EN};
    use tower::ServiceExt;

    use crate::{
        application::usecase::create_user_usecase::CreateUserUsecase,
        infrastructure::repository::{build_db_connection, user_repository::UserRepositoryImpl},
        presentation::router::user_router::UserRouter,
    };

    use super::*;

    #[tokio::test]
    async fn test_create_user_router() {
        let db = build_db_connection().await.unwrap();
        let user_repository = UserRepositoryImpl::new(db);
        let create_user_usecase = CreateUserUsecase::new(Arc::new(user_repository));
        let handler = CreateUserHandler::new(Arc::new(create_user_usecase));
        let router = UserRouter::new(Arc::new(handler));

        let app = Router::new().merge(router.routes());

        let name: String = Name(EN).fake();
        let body = format!(r#"{{"name":"{name}"}}"#);
        let request = Request::builder()
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .method(Method::POST)
            .uri("/users")
            .body(Body::from(body))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }
}
