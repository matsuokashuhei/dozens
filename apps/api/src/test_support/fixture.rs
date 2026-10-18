use crate::infrastructure::repository::{
    user_identity_repository::UserIdentityRecord, user_repository::UserRecord,
};

use super::{
    cognito::delete_cognito_users,
    db::{build_db_connection, delete_records},
};

static MIGRATIONS: tokio::sync::OnceCell<()> = tokio::sync::OnceCell::const_new();

pub(crate) async fn set_up() {
    let _ = tracing_subscriber::fmt::try_init();
    migrate().await;
    tear_down().await;
}

pub(crate) async fn set_up_with_cognito() {
    let _ = tracing_subscriber::fmt::try_init();
    migrate().await;
    tear_down_with_cognito().await;
}

pub(crate) async fn tear_down() {
    let mut db = build_db_connection().await.unwrap();
    delete_records::<UserIdentityRecord>(&mut db).await;
    delete_records::<UserRecord>(&mut db).await;
}

pub(crate) async fn tear_down_with_cognito() {
    tear_down().await;
    delete_cognito_users().await;
}

async fn migrate() {
    let db = build_db_connection().await.unwrap();
    MIGRATIONS
        .get_or_init(|| async {
            toasty::embed_migrations!().apply(&db).await.unwrap();
        })
        .await;
}
