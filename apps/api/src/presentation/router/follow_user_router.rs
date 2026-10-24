use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::post};

use crate::{
    application::usecase::follow_user_usecase::FollowUserBody,
    presentation::{
        handler::follow_user_handler::FollowUserHandler,
        middleware::{json_validator::JsonValidator, user_authenticator::UserAuthenticator},
    },
};

pub struct FollowUserRouter {
    pub follow_user: Arc<FollowUserHandler>,
}

impl FollowUserRouter {
    pub fn new(follow_user: Arc<FollowUserHandler>) -> Self {
        Self { follow_user }
    }

    pub fn routes(&self) -> Router {
        Router::new().route(
            "/follow",
            post({
                let handler = self.follow_user.clone();
                move |user_authenticator: UserAuthenticator,
                      input: JsonValidator<FollowUserBody>| {
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        Extension, Router,
        body::Body,
        http::{
            Method, Request, StatusCode,
            header::{AUTHORIZATION, CONTENT_TYPE},
        },
    };
    use serde_json::json;
    use toasty::Db;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        application::usecase::{
            authenticate_user_usecase::AuthenticateUserUsecase,
            follow_user_usecase::FollowUserUsecase,
        },
        domain::repository::{
            user_relationship_repository::UserRelationshipRepository,
            user_repository::UserRepository,
        },
        infrastructure::{
            repository::{
                user_relationship_repository::UserRelationshipRepositoryImpl,
                user_repository::UserRepositoryImpl,
            },
            service::local_token_decoder::{LocalTokenDecoder, issue_access_token},
        },
        presentation::{
            handler::follow_user_handler::FollowUserHandler, middleware::token_authenticator,
        },
        test_support::{
            DB_LOCK, build_db_connection, create_user, create_user_identity, set_up, tear_down,
        },
    };

    use super::*;

    fn build_app(db: &Db) -> Router {
        let user_repository: Arc<dyn UserRepository> =
            Arc::new(UserRepositoryImpl::new(db.clone()));
        let user_relationship_repository: Arc<dyn UserRelationshipRepository> =
            Arc::new(UserRelationshipRepositoryImpl::new(db.clone()));
        let authenticate_user_usecase = Arc::new(AuthenticateUserUsecase::new(user_repository));
        let follow_user_usecase = FollowUserUsecase::new(user_relationship_repository);
        let follow_user_handler = FollowUserHandler::new(Arc::new(follow_user_usecase));
        let router = FollowUserRouter::new(Arc::new(follow_user_handler));
        Router::new()
            .merge(router.routes())
            .layer(token_authenticator::extension(Arc::new(
                LocalTokenDecoder::new(),
            )))
            .layer(Extension(authenticate_user_usecase))
    }

    fn follow_request(
        token: Option<&str>,
        followee_id: crate::domain::model::user::UserId,
    ) -> Request<Body> {
        let mut builder = Request::builder()
            .method(Method::POST)
            .uri("/follow")
            .header(CONTENT_TYPE, "application/json");
        if let Some(token) = token {
            builder = builder.header(AUTHORIZATION, format!("Bearer {token}"));
        }
        builder
            .body(Body::from(
                serde_json::to_string(&json!({ "followee_id": followee_id })).unwrap(),
            ))
            .unwrap()
    }

    async fn error_body(response: axum::response::Response) -> serde_json::Value {
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    #[tokio::test]
    async fn test_follow_user_succeeds() {
        let _guard = DB_LOCK.lock().await;
        set_up().await;
        let mut db = build_db_connection().await.unwrap();
        let sub = Uuid::now_v7().to_string();
        let follower_id = create_user(&mut db).await;
        create_user_identity(&mut db, follower_id, &sub).await;
        let followee_id = create_user(&mut db).await;
        let app = build_app(&db);
        let token = issue_access_token(&sub, 3600).unwrap();

        let response = app
            .oneshot(follow_request(Some(&token), followee_id))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let repository = UserRelationshipRepositoryImpl::new(db.clone());
        let relationship = repository
            .get_user_relationship(follower_id, followee_id)
            .await;
        assert!(relationship.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    async fn test_follow_user_succeeds_when_relationship_already_exists() {
        let _guard = DB_LOCK.lock().await;
        set_up().await;
        let mut db = build_db_connection().await.unwrap();
        let sub = Uuid::now_v7().to_string();
        let follower_id = create_user(&mut db).await;
        create_user_identity(&mut db, follower_id, &sub).await;
        let followee_id = create_user(&mut db).await;
        let repository = UserRelationshipRepositoryImpl::new(db.clone());
        repository
            .create_user_relationship(
                crate::domain::model::user_relationship::UserRelationship::new(
                    follower_id,
                    followee_id,
                )
                .unwrap(),
            )
            .await
            .unwrap();
        let app = build_app(&db);
        let token = issue_access_token(&sub, 3600).unwrap();

        let response = app
            .oneshot(follow_request(Some(&token), followee_id))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        tear_down().await;
    }

    #[tokio::test]
    async fn test_follow_user_fails_when_follower_equals_followee() {
        let _guard = DB_LOCK.lock().await;
        set_up().await;
        let mut db = build_db_connection().await.unwrap();
        let sub = Uuid::now_v7().to_string();
        let follower_id = create_user(&mut db).await;
        create_user_identity(&mut db, follower_id, &sub).await;
        let app = build_app(&db);
        let token = issue_access_token(&sub, 3600).unwrap();

        let response = app
            .oneshot(follow_request(Some(&token), follower_id))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = error_body(response).await;
        assert_eq!(body["code"], "bad_request");
        assert!(body["message"].is_string());
        tear_down().await;
    }

    #[tokio::test]
    async fn test_follow_user_fails_when_token_is_missing() {
        let _guard = DB_LOCK.lock().await;
        set_up().await;
        let mut db = build_db_connection().await.unwrap();
        let followee_id = create_user(&mut db).await;
        let app = build_app(&db);

        let response = app
            .oneshot(follow_request(None, followee_id))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body = error_body(response).await;
        assert_eq!(body["code"], "unauthorized");
        assert!(body["message"].is_string());
        tear_down().await;
    }
}
