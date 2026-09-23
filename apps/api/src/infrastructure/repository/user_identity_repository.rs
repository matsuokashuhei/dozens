use async_trait::async_trait;
use toasty::Db;
use uuid::Uuid;

use crate::{
    domain::{
        model::user_identity::UserIdentity,
        repository::{errors::RepositoryError, user_identity_repository::UserIdentityRepository},
    },
    infrastructure::repository::user_repository::UserRecord,
};

#[derive(Debug, toasty::Model)]
#[table = "user_identities"]
#[unique(iss, sub)]
pub(crate) struct UserIdentityRecord {
    #[key]
    pub id: Uuid,
    #[index]
    pub user_id: Uuid,
    // TODO: When FK is supported, use it instead of the deferred relationship
    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::Deferred<UserRecord>,
    pub iss: String,
    pub sub: String,
}

impl From<UserIdentityRecord> for UserIdentity {
    fn from(record: UserIdentityRecord) -> Self {
        UserIdentity {
            id: record.id,
            user_id: record.user_id,
            iss: record.iss,
            sub: record.sub,
        }
    }
}

pub struct UserIdentityRepositoryImpl {
    db: Db,
}

impl UserIdentityRepositoryImpl {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserIdentityRepository for UserIdentityRepositoryImpl {
    async fn create_user_identity(
        &self,
        user_identity: UserIdentity,
    ) -> Result<UserIdentity, RepositoryError> {
        let mut db = self.db.clone();
        let record = toasty::create!(UserIdentityRecord {
            id: user_identity.id,
            user_id: user_identity.user_id,
            iss: user_identity.iss,
            sub: user_identity.sub,
        })
        .exec(&mut db)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(record.into())
    }
}
