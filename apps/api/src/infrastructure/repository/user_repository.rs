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

#[derive(Debug, toasty::Model)]
#[table = "users"]
pub(crate) struct UserRecord {
    #[key]
    pub id: Uuid,
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
            id: record.id,
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
            id: user.id,
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

    use crate::infrastructure::{
        service::cognito_identity_provider::CognitoIdentityProvider,
        test_support::{self, set_up, tear_down},
    };

    use super::*;

    #[tokio::test]
    async fn test_get_user_by_sub() {
        set_up().await;
        let mut db = test_support::build_db_connection().await.unwrap();
        let user_repository = UserRepositoryImpl::new(db.clone());
        {
            let user = UserRecord::create()
                .id(Uuid::now_v7())
                .name(Name(EN).fake::<String>())
                .exec(&mut db)
                .await
                .unwrap();
            UserIdentityRecord::create()
                .id(Uuid::now_v7())
                .user_id(user.id)
                .iss(CognitoIdentityProvider::issuer())
                .sub(Uuid::now_v7().to_string())
                .exec(&mut db)
                .await
                .unwrap();
        }
        let user = UserRecord::create()
            .id(Uuid::now_v7())
            .name(Name(EN).fake::<String>())
            .exec(&mut db)
            .await
            .unwrap();
        let user_identity = UserIdentityRecord::create()
            .id(Uuid::now_v7())
            .user_id(user.id)
            .iss(CognitoIdentityProvider::issuer())
            .sub(Uuid::now_v7().to_string())
            .exec(&mut db)
            .await
            .unwrap();
        let user = user_repository
            .get_user_by_sub(&user_identity.sub)
            .await
            .unwrap();
        assert_eq!(user.id, user.id);
        tear_down().await;
    }
}
