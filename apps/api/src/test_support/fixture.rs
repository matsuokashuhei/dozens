use fake::{Fake, faker::name::raw::Name, locales::EN};
use toasty::Db;
use uuid::Uuid;

use crate::{
    domain::model::user,
    infrastructure::{
        repository::{
            user_identity_repository::UserIdentityRecord,
            user_repository::{UserId, UserRecord},
        },
        service::local_token_decoder::issuer,
    },
};

use super::{
    cognito::delete_cognito_users,
    db::{build_db_connection, delete_records},
};

static MIGRATIONS: tokio::sync::OnceCell<()> = tokio::sync::OnceCell::const_new();

/// Serializes every DB-backed test: `set_up()` / `tear_down()` wipe shared tables, so tests
/// that touch the database must not run concurrently with each other.
pub(crate) static DB_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

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

pub(crate) async fn create_user(db: &mut Db) -> user::UserId {
    let user_id = user::UserId::new();
    UserRecord::create()
        .id(UserId::from(user_id))
        .name(Name(EN).fake::<String>())
        .exec(db)
        .await
        .unwrap();
    user_id
}

pub(crate) async fn create_user_identity(db: &mut Db, user_id: user::UserId, sub: &str) {
    UserIdentityRecord::create()
        .id(Uuid::now_v7())
        .user_id(UserId::from(user_id))
        .iss(issuer())
        .sub(sub.to_string())
        .exec(db)
        .await
        .unwrap();
}

async fn migrate() {
    let db = build_db_connection().await.unwrap();
    MIGRATIONS
        .get_or_init(|| async {
            toasty::embed_migrations!().apply(&db).await.unwrap();
        })
        .await;
}
