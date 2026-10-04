use tracing::info;

use crate::infrastructure::repository::{
    user_identity_repository::UserIdentityRecord, user_repository::UserRecord,
};

use super::{
    cognito::delete_cognito_users,
    db::{build_db_connection, delete_records},
};

pub(crate) async fn set_up() {
    let _ = tracing_subscriber::fmt::try_init();
    info!("Setting up test environment");
    let db = build_db_connection().await.unwrap();
    toasty::embed_migrations!().apply(&db).await.unwrap();
    tear_down().await;
}

pub(crate) async fn tear_down() {
    let mut db = build_db_connection().await.unwrap();
    delete_records::<UserIdentityRecord>(&mut db).await;
    delete_records::<UserRecord>(&mut db).await;
    delete_cognito_users().await;
}
