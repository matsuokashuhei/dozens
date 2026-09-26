use std::sync::Arc;

use anyhow::Result;
use api::{
    application::usecase::{
        confirm_sign_in_usecase::ConfirmSignInUsecase,
        confirm_sign_up_usecase::ConfirmSignUpUsecase, sign_in_usecase::SignInUsecase,
        sign_up_usecase::SignUpUsecase,
    },
    infrastructure::{
        repository::{
            build_db_connection, user_identity_repository::UserIdentityRepositoryImpl,
            user_repository::UserRepositoryImpl,
        },
        service::cognito_identity_provider::CognitoIdentityProvider,
    },
    presentation::{
        handler::{
            confirm_sign_in_handler::ConfirmSignInHandler,
            confirm_sign_up_handler::ConfirmSignUpHandler, sign_in_handler::SignInHandler,
            sign_up_handler::SignUpHandler,
        },
        router::{
            confirm_sign_in_router::ConfirmSignInRouter,
            confirm_sign_up_router::ConfirmSignUpRouter, sign_in_router::SignInRouter,
            sign_up_router::SignUpRouter,
        },
    },
};
use axum::{Router, http::StatusCode, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;
    let db = build_db_connection("postgresql://postgres:postgres@localhost:5432/dozens").await?;
    // sign up
    let client = aws_sdk_cognitoidentityprovider::Client::new(&aws_config);
    let user_repository = UserRepositoryImpl::new(db.clone());
    let identity_provider = CognitoIdentityProvider::new(client);
    let user_identity_repository = UserIdentityRepositoryImpl::new(db.clone());
    let sign_up_usecase = SignUpUsecase::new(
        Arc::new(identity_provider),
        Arc::new(user_repository),
        Arc::new(user_identity_repository),
    );
    let sign_up_handler = SignUpHandler::new(Arc::new(sign_up_usecase));
    let sign_up_router = SignUpRouter::new(Arc::new(sign_up_handler));
    // confirm sign up
    let client = aws_sdk_cognitoidentityprovider::Client::new(&aws_config);
    let identity_provider = CognitoIdentityProvider::new(client);
    let confirm_sign_up_usecase = ConfirmSignUpUsecase::new(Arc::new(identity_provider));
    let confirm_sign_up_handler = ConfirmSignUpHandler::new(Arc::new(confirm_sign_up_usecase));
    let confirm_sign_up_router = ConfirmSignUpRouter::new(Arc::new(confirm_sign_up_handler));
    // sign in
    let client = aws_sdk_cognitoidentityprovider::Client::new(&aws_config);
    let identity_provider = CognitoIdentityProvider::new(client);
    let sign_in_usecase = SignInUsecase::new(Arc::new(identity_provider));
    let sign_in_handler = SignInHandler::new(Arc::new(sign_in_usecase));
    let sign_in_router = SignInRouter::new(Arc::new(sign_in_handler));
    // confirm sign in
    let client = aws_sdk_cognitoidentityprovider::Client::new(&aws_config);
    let identity_provider = CognitoIdentityProvider::new(client);
    let confirm_sign_in_usecase = ConfirmSignInUsecase::new(Arc::new(identity_provider));
    let confirm_sign_in_handler = ConfirmSignInHandler::new(Arc::new(confirm_sign_in_usecase));
    let confirm_sign_in_router = ConfirmSignInRouter::new(Arc::new(confirm_sign_in_handler));
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| async { StatusCode::OK }))
        .merge(sign_up_router.routes())
        .merge(confirm_sign_up_router.routes())
        .merge(sign_in_router.routes())
        .merge(confirm_sign_in_router.routes());
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
