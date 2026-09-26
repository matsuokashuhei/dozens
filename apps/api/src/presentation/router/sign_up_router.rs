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
    // use std::env;

    // use axum::{
    //     Router,
    //     body::Body,
    //     http::{HeaderValue, Method, Request, StatusCode, header::CONTENT_TYPE},
    // };
    // use tower::ServiceExt;

    // use crate::infrastructure::{
    //     repository::user_identity_repository::UserIdentityRecord,
    //     test_support::{build_db_connection, build_sign_up_router, set_up, tear_down},
    // };

    // #[tokio::test]
    // async fn test_sign_up_router() {
    //     set_up().await;

    //     let router = build_sign_up_router().await;
    //     let app = Router::new().merge(router.routes());

    //     let email = "matsuokashuheiii+test1@gmail.com";
    //     let body = format!(r#"{{"email":"{email}"}}"#);
    //     let request = Request::builder()
    //         .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
    //         .method(Method::POST)
    //         .uri("/sign_up")
    //         .body(Body::from(body))
    //         .unwrap();
    //     let response = app.oneshot(request).await.unwrap();

    //     assert_eq!(response.status(), StatusCode::CREATED);
    //     let cognito_user = get_cognito_user(email).await;
    //     let cognito_email = cognito_user
    //         .user_attributes()
    //         .iter()
    //         .find(|attribute| attribute.name() == "email")
    //         .and_then(|attribute| attribute.value());
    //     assert_eq!(cognito_email, Some(email));
    //     let mut db = build_db_connection().await.unwrap();
    //     let user_identity = UserIdentityRecord::filter_by_iss_and_sub(
    //         env::var("AWS_COGNITO_USER_POOL_ID").unwrap(),
    //         cognito_user.username(),
    //     )
    //     .get(&mut db)
    //     .await;
    //     assert!(user_identity.is_ok());

    //     tear_down().await;
    // }
}
