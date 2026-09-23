use std::sync::Arc;

use anyhow::Result;
use api::{
    application::usecase::create_user_usecase::CreateUserUsecase,
    infrastructure::repository::build_db_connection,
    infrastructure::repository::user_repository::UserRepositoryImpl,
    presentation::{
        handler::create_user_handler::CreateUserHandler, router::user_router::UserRouter,
    },
};
use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let db = build_db_connection().await?;
    let user_repository = UserRepositoryImpl::new(db);
    let create_user_usecase = CreateUserUsecase::new(Arc::new(user_repository));
    let create_user_handler = CreateUserHandler::new(Arc::new(create_user_usecase));
    let user_router = UserRouter::new(Arc::new(create_user_handler));
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| async { "OK" }))
        .merge(user_router.routes());
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
