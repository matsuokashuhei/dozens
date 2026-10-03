use std::sync::Arc;

use anyhow::Result;
use api::{
    application::{
        service::identity_provider::IdentityProvider,
        usecase::{
            change_email_usecase::ChangeEmailUsecase,
            confirm_change_email_usecase::ConfirmChangeEmailUsecase,
            authenticate_user_usecase::AuthenticateUserUsecase,
            confirm_sign_in_usecase::ConfirmSignInUsecase,
            confirm_sign_up_usecase::ConfirmSignUpUsecase, sign_in_usecase::SignInUsecase,
            sign_out_usecase::SignOutUsecase, sign_up_usecase::SignUpUsecase,
        },
    },
    domain::repository::user_repository::UserRepository,
    infrastructure::{
        repository::{
            build_db_connection, user_identity_repository::UserIdentityRepositoryImpl,
            user_repository::UserRepositoryImpl,
        },
        service::cognito_identity_provider::CognitoIdentityProvider,
    },
    presentation::{
        handler::{
            change_email_handler::ChangeEmailHandler,
            confirm_change_email_handler::ConfirmChangeEmailHandler,
            confirm_sign_in_handler::ConfirmSignInHandler,
            confirm_sign_up_handler::ConfirmSignUpHandler, get_user_handler::GetUserHandler,
            sign_in_handler::SignInHandler, sign_out_handler::SignOutHandler,
            sign_up_handler::SignUpHandler,
        },
        middleware::token_authenticator,
        router::{
            change_email_router::ChangeEmailRouter,
            confirm_change_email_router::ConfirmChangeEmailRouter,
            confirm_sign_in_router::ConfirmSignInRouter,
            confirm_sign_up_router::ConfirmSignUpRouter, get_user_router::GetUserRouter,
            sign_in_router::SignInRouter, sign_out_router::SignOutRouter,
            sign_up_router::SignUpRouter,
        },
    },
};
use axum::{Extension, Router, http::StatusCode, routing::get};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;
    let db = build_db_connection("postgresql://postgres:postgres@localhost:5432/dozens").await?;
    let user_repository: Arc<dyn UserRepository> = Arc::new(UserRepositoryImpl::new(db.clone()));
    // sign up
    let client = aws_sdk_cognitoidentityprovider::Client::new(&aws_config);
    let identity_provider = CognitoIdentityProvider::new(client);
    let decoder = identity_provider.build_token_decoder().await?;
    let user_identity_repository = UserIdentityRepositoryImpl::new(db.clone());
    let sign_up_usecase = SignUpUsecase::new(
        Arc::new(identity_provider),
        user_repository.clone(),
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
    // sign out
    let client = aws_sdk_cognitoidentityprovider::Client::new(&aws_config);
    let identity_provider = CognitoIdentityProvider::new(client);
    let sign_out_usecase = SignOutUsecase::new(Arc::new(identity_provider));
    let sign_out_handler = SignOutHandler::new(Arc::new(sign_out_usecase));
    let sign_out_router = SignOutRouter::new(Arc::new(sign_out_handler));
    // change email
    let client = aws_sdk_cognitoidentityprovider::Client::new(&aws_config);
    let identity_provider = CognitoIdentityProvider::new(client);
    let change_email_usecase = ChangeEmailUsecase::new(Arc::new(identity_provider));
    let change_email_handler = ChangeEmailHandler::new(Arc::new(change_email_usecase));
    let change_email_router = ChangeEmailRouter::new(Arc::new(change_email_handler));
    // confirm change email
    let client = aws_sdk_cognitoidentityprovider::Client::new(&aws_config);
    let identity_provider = CognitoIdentityProvider::new(client);
    let confirm_change_email_usecase = ConfirmChangeEmailUsecase::new(Arc::new(identity_provider));
    let confirm_change_email_handler =
        ConfirmChangeEmailHandler::new(Arc::new(confirm_change_email_usecase));
    let confirm_change_email_router =
        ConfirmChangeEmailRouter::new(Arc::new(confirm_change_email_handler));
    // get user
    let authenticate_user_usecase = Arc::new(AuthenticateUserUsecase::new(user_repository.clone()));
    let get_user_handler = GetUserHandler::new();
    let get_user_router = GetUserRouter::new(Arc::new(get_user_handler));
    // build app
    let app = Router::new()
        .route("/health", get(|| async { StatusCode::OK }))
        .merge(sign_up_router.routes())
        .merge(confirm_sign_up_router.routes())
        .merge(sign_in_router.routes())
        .merge(confirm_sign_in_router.routes())
        .merge(sign_out_router.routes())
        .merge(change_email_router.routes())
        .merge(confirm_change_email_router.routes())
        .merge(get_user_router.routes())
        .layer(token_authenticator::extension(decoder))
        .layer(Extension(authenticate_user_usecase))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
