use std::sync::Arc;

use axum::{Router, extract::Path, response::IntoResponse, routing::delete};

use crate::{
    domain::model::user::UserId,
    presentation::{
        handler::unfollow_user_handler::UnfollowUserHandler,
        middleware::user_authenticator::UserAuthenticator,
    },
};

pub struct UnfollowUserRouter {
    pub unfollow_user: Arc<UnfollowUserHandler>,
}

impl UnfollowUserRouter {
    pub fn new(unfollow_user: Arc<UnfollowUserHandler>) -> Self {
        Self { unfollow_user }
    }

    pub fn routes(&self) -> Router {
        Router::new().route(
            "/follow/{followee_id}",
            delete({
                let handler = self.unfollow_user.clone();
                move |user_authenticator: UserAuthenticator, path: Path<UserId>| {
                    let handler = handler.clone();
                    async move {
                        handler
                            .handle(user_authenticator, path)
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
        http::{Method, Request, StatusCode, header::AUTHORIZATION},
    };
    use toasty::Db;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        application::usecase::{
            authenticate_user_usecase::AuthenticateUserUsecase,
            unfollow_user_usecase::UnfollowUserUsecase,
        },
        domain::{
            model::user_relationship::UserRelationship,
            repository::{
                user_relationship_repository::UserRelationshipRepository,
                user_repository::UserRepository,
            },
        },
        infrastructure::{
            repository::{
                user_relationship_repository::UserRelationshipRepositoryImpl,
                user_repository::UserRepositoryImpl,
            },
            service::local_token_decoder::{LocalTokenDecoder, issue_access_token},
        },
        presentation::{
            handler::unfollow_user_handler::UnfollowUserHandler, middleware::token_authenticator,
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
        let unfollow_user_usecase = UnfollowUserUsecase::new(user_relationship_repository);
        let unfollow_user_handler = UnfollowUserHandler::new(Arc::new(unfollow_user_usecase));
        let router = UnfollowUserRouter::new(Arc::new(unfollow_user_handler));
        Router::new()
            .merge(router.routes())
            .layer(token_authenticator::extension(Arc::new(
                LocalTokenDecoder::new(),
            )))
            .layer(Extension(authenticate_user_usecase))
    }

    fn unfollow_request(
        token: &str,
        followee_id: crate::domain::model::user::UserId,
    ) -> Request<Body> {
        Request::builder()
            .method(Method::DELETE)
            .uri(format!("/follow/{followee_id}"))
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .body(Body::empty())
            .unwrap()
    }

    #[tokio::test]
    async fn test_unfollow_user_succeeds() {
        let _guard = DB_LOCK.lock().await;
        set_up().await;
        let mut db = build_db_connection().await.unwrap();
        let sub = Uuid::now_v7().to_string();
        let follower_id = create_user(&mut db).await;
        create_user_identity(&mut db, follower_id, &sub).await;
        let followee_id = create_user(&mut db).await;
        let repository = UserRelationshipRepositoryImpl::new(db.clone());
        repository
            .create_user_relationship(UserRelationship::new(follower_id, followee_id).unwrap())
            .await
            .unwrap();
        let app = build_app(&db);
        let token = issue_access_token(&sub, 3600).unwrap();

        let response = app
            .oneshot(unfollow_request(&token, followee_id))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let relationship = repository
            .get_user_relationship(follower_id, followee_id)
            .await;
        assert!(relationship.is_err());
        tear_down().await;
    }

    #[tokio::test]
    async fn test_unfollow_user_succeeds_when_relationship_does_not_exist() {
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
            .oneshot(unfollow_request(&token, followee_id))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        tear_down().await;
    }
}
