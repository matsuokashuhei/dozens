use async_trait::async_trait;
use jiff::Timestamp;
use toasty::Db;
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
}
