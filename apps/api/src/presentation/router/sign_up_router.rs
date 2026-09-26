use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::post};

use crate::{
    application::usecase::sign_up_usecase::SignUpInput,
    presentation::{
        handler::sign_up_handler::SignUpHandler, middleware::json_validator::JsonValidator,
    },
};

pub struct SignUpRouter {
    pub sign_up: Arc<SignUpHandler>,
}

impl SignUpRouter {
    pub fn new(sign_up: Arc<SignUpHandler>) -> Self {
        Self { sign_up }
    }

    pub fn routes(&self) -> Router {
        Router::new().route(
            "/sign_up",
            post({
                let handler = self.sign_up.clone();
                move |input: JsonValidator<SignUpInput>| {
                    let handler = handler.clone();
                    async move { handler.handle(input).await.into_response() }
                }
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{env, sync::Arc};

    use axum::{
        Router,
        body::Body,
        http::{HeaderValue, Method, Request, StatusCode, header::CONTENT_TYPE},
    };
    use tower::ServiceExt;

    use crate::{
        application::usecase::sign_up_usecase::SignUpUsecase,
        domain::model::email::Email,
        infrastructure::{
            repository::{
                user_identity_repository::{UserIdentityRecord, UserIdentityRepositoryImpl},
                user_repository::UserRepositoryImpl,
            },
            test_support::{
                TEST_EMAILS, build_cognito_identity_provider, build_db_connection, set_up,
                tear_down,
            },
        },
        presentation::{
            handler::sign_up_handler::SignUpHandler, router::sign_up_router::SignUpRouter,
        },
    };

    #[tokio::test]
    async fn test_sign_up_router() {
        set_up().await;

        let db = build_db_connection().await.unwrap();
        let user_repository = UserRepositoryImpl::new(db.clone());
        let user_identity_repository = UserIdentityRepositoryImpl::new(db.clone());
        let identity_provider = build_cognito_identity_provider().await;
        let sign_up_usecase = SignUpUsecase::new(
            Arc::new(identity_provider),
            Arc::new(user_repository),
            Arc::new(user_identity_repository),
        );
        let handler = SignUpHandler::new(Arc::new(sign_up_usecase));
        let router = SignUpRouter::new(Arc::new(handler));
        let app = Router::new().merge(router.routes());

        let email = Email::new(TEST_EMAILS[0]).unwrap();
        let body = format!(r#"{{"email":"{}"}}"#, email.as_str());
        let request = Request::builder()
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .method(Method::POST)
            .uri("/sign_up")
            .body(Body::from(body))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let identity_provider = build_cognito_identity_provider().await;
        let result = identity_provider.request_admin_get_user(email).await;
        assert!(result.is_ok());
        let mut db = build_db_connection().await.unwrap();
        let user_identity = UserIdentityRecord::filter_by_iss_and_sub(
            env::var("AWS_COGNITO_USER_POOL_ID").unwrap(),
            result.unwrap().username(),
        )
        .get(&mut db)
        .await;
        assert!(user_identity.is_ok());

        tear_down().await;
    }
}
