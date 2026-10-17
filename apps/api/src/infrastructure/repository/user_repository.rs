use async_trait::async_trait;
use jiff::Timestamp;
use toasty::Db;
use toasty::stmt::{List, Query};
use uuid::Uuid;

use crate::{
    domain::{
        model::user::User,
        repository::{errors::RepositoryError, user_repository::UserRepository},
    },
    infrastructure::repository::user_identity_repository::UserIdentityRecord,
};

/// Storage-side user id, kept separate from the domain type so the domain stays toasty-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, toasty::Embed)]
pub(crate) struct UserId(Uuid);

impl From<crate::domain::model::user_id::UserId> for UserId {
    fn from(id: crate::domain::model::user_id::UserId) -> Self {
        Self(id.as_uuid())
    }
}

impl From<UserId> for crate::domain::model::user_id::UserId {
    fn from(id: UserId) -> Self {
        Self::from(id.0)
    }
}

#[derive(Debug, toasty::Model)]
#[table = "users"]
pub(crate) struct UserRecord {
    #[key]
    pub id: UserId,
    pub name: String,
    #[auto]
    created_at: Timestamp,
    #[auto]
    updated_at: Timestamp,

    #[has_many(pair = user)]
    pub user_identities: toasty::Deferred<Vec<UserIdentityRecord>>,
}

impl From<UserRecord> for User {
    fn from(record: UserRecord) -> Self {
        User {
            id: record.id.into(),
            name: record.name,
        }
    }
}

pub struct UserRepositoryImpl {
    db: Db,
}

impl UserRepositoryImpl {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, user: User) -> Result<User, RepositoryError> {
        let mut db = self.db.clone();
        let record = toasty::create!(UserRecord {
            id: UserId::from(user.id),
            name: user.name
        })
        .exec(&mut db)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(record.into())
    }

    async fn get_user_by_sub(&self, sub: &str) -> Result<User, RepositoryError> {
        let mut db = self.db.clone();
        let record = Query::<List<UserRecord>>::all()
            .filter(
                UserRecord::fields()
                    .user_identities()
                    .any(UserIdentityRecord::fields().sub().eq(sub)),
            )
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
            .ok_or_else(|| RepositoryError::NotFound("User not found for sub".into()))?;
        Ok(record.into())
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, faker::name::raw::Name, locales::EN};
    use uuid::Uuid;

    use crate::{
        domain::model::user_id::UserId as DomainUserId,
        infrastructure::service::cognito_identity_provider::CognitoIdentityProvider,
        test_support::{self, set_up, tear_down},
    };

    use super::*;

    /// `set_up()` / `tear_down()` wipe shared tables in the test database, so these DB tests
    /// must not run concurrently with each other.
    static DB_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

    async fn build_db_connection() -> (Db, UserRepositoryImpl) {
        let db = test_support::build_db_connection().await.unwrap();
        let repository = UserRepositoryImpl::new(db.clone());
        (db, repository)
    }

    async fn create_user_record(db: &mut Db, id: DomainUserId) -> UserRecord {
        UserRecord::create()
            .id(UserId::from(id))
            .name(Name(EN).fake::<String>())
            .exec(db)
            .await
            .unwrap()
    }

    async fn create_user_identity_record(db: &mut Db, user_id: UserId) -> UserIdentityRecord {
        UserIdentityRecord::create()
            .id(Uuid::now_v7())
            .user_id(user_id)
            .iss(CognitoIdentityProvider::issuer())
            .sub(Uuid::now_v7().to_string())
            .exec(db)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_create_user_with_success() {
        let _guard = DB_LOCK.lock().await;
        set_up().await;
        let (mut db, repository) = build_db_connection().await;
        let user = User::new(Name(EN).fake::<String>());

        let created = repository.create_user(user.clone()).await.unwrap();

        assert_eq!(created.id, user.id);
        assert_eq!(created.name, user.name);
        let records = Query::<List<UserRecord>>::all()
            .exec(&mut db)
            .await
            .unwrap();
        assert!(
            records
                .iter()
                .any(|r| r.id == UserId::from(user.id) && r.name == user.name)
        );
        tear_down().await;
    }

    #[tokio::test]
    async fn test_get_user_by_sub_with_success() {
        let _guard = DB_LOCK.lock().await;
        set_up().await;
        let (mut db, repository) = build_db_connection().await;
        // an unrelated user with its own identity must not be returned
        let unrelated = create_user_record(&mut db, DomainUserId::new()).await;
        create_user_identity_record(&mut db, unrelated.id).await;
        let target = create_user_record(&mut db, DomainUserId::new()).await;
        let expected_name = target.name.clone();
        let user_identity = create_user_identity_record(&mut db, target.id).await;

        let user = repository
            .get_user_by_sub(&user_identity.sub)
            .await
            .unwrap();

        assert_eq!(user.id, DomainUserId::from(target.id));
        assert_eq!(user.name, expected_name);
        tear_down().await;
    }

    #[tokio::test]
    async fn test_get_user_by_sub_with_user_not_found() {
        let _guard = DB_LOCK.lock().await;
        set_up().await;
        let (_db, repository) = build_db_connection().await;
        let missing_sub = Uuid::now_v7().to_string();

        let user = repository.get_user_by_sub(&missing_sub).await;

        assert!(matches!(user, Err(RepositoryError::NotFound(_))));
        tear_down().await;
    }
}
