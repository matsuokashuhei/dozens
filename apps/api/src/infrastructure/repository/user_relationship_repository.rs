use async_trait::async_trait;
use jiff::Timestamp;
use toasty::Db;
use uuid::Uuid;

use crate::domain::{
    model::{user_id::UserId, user_relationship::UserRelationship},
    repository::{
        errors::RepositoryError, user_relationship_repository::UserRelationshipRepository,
    },
};

#[derive(Debug, toasty::Model)]
#[table = "user_relationships"]
#[unique(follower_id, followee_id)]
pub(crate) struct UserRelationshipRecord {
    #[key]
    pub id: Uuid,
    #[index]
    pub follower_id: Uuid,
    #[index]
    pub followee_id: Uuid,
    #[auto]
    created_at: Timestamp,
}

impl From<UserRelationshipRecord> for UserRelationship {
    fn from(record: UserRelationshipRecord) -> Self {
        UserRelationship {
            id: record.id,
            follower_id: record.follower_id.into(),
            followee_id: record.followee_id.into(),
        }
    }
}

pub struct UserRelationshipRepositoryImpl {
    db: Db,
}

impl UserRelationshipRepositoryImpl {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRelationshipRepository for UserRelationshipRepositoryImpl {
    async fn create_user_relationship(
        &self,
        user_relationship: UserRelationship,
    ) -> Result<UserRelationship, RepositoryError> {
        let mut db = self.db.clone();
        let record = toasty::create!(UserRelationshipRecord {
            id: user_relationship.id,
            follower_id: Uuid::from(user_relationship.follower_id),
            followee_id: Uuid::from(user_relationship.followee_id),
        })
        .exec(&mut db)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(record.into())
    }

    async fn get_user_relationship(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<UserRelationship, RepositoryError> {
        let mut db = self.db.clone();
        let record =
            UserRelationshipRecord::filter_by_follower_id_and_followee_id(
                Uuid::from(follower_id),
                Uuid::from(followee_id),
            )
            .first()
                .exec(&mut db)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
                .ok_or_else(|| RepositoryError::NotFound("UserRelationship not found".into()))?;
        Ok(record.into())
    }

    async fn list_user_relationships_by_follower(
        &self,
        follower_id: UserId,
    ) -> Result<Vec<UserRelationship>, RepositoryError> {
        let mut db = self.db.clone();
        let records = UserRelationshipRecord::filter_by_follower_id(Uuid::from(follower_id))
            .exec(&mut db)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(records.into_iter().map(Into::into).collect())
    }

    async fn list_user_relationships_by_followee(
        &self,
        followee_id: UserId,
    ) -> Result<Vec<UserRelationship>, RepositoryError> {
        let mut db = self.db.clone();
        let records = UserRelationshipRecord::filter_by_followee_id(Uuid::from(followee_id))
            .exec(&mut db)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(records.into_iter().map(Into::into).collect())
    }

    async fn delete_user_relationship(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<(), RepositoryError> {
        let mut db = self.db.clone();
        UserRelationshipRecord::delete_by_follower_id_and_followee_id(
            &mut db,
            Uuid::from(follower_id),
            Uuid::from(followee_id),
        )
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, faker::name::raw::Name, locales::EN};
    use toasty::stmt::{List, Query};
    use uuid::Uuid;

    use crate::{
        infrastructure::repository::user_repository::{UserId as RecordUserId, UserRecord},
        test_support::{self, set_up, tear_down},
    };

    use super::*;

    /// `set_up()` / `tear_down()` wipe shared tables in the test database, so these DB tests
    /// must not run concurrently with each other.
    static DB_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

    async fn set_up_tests_db() -> (Db, UserRelationshipRepositoryImpl) {
        set_up().await;
        let mut db = test_support::build_db_connection().await.unwrap();
        // tear_down() does not clean user_relationships; drop leftovers from earlier runs
        Query::<List<UserRelationshipRecord>>::all()
            .delete()
            .exec(&mut db)
            .await
            .unwrap();
        let repository = UserRelationshipRepositoryImpl::new(db.clone());
        (db, repository)
    }

    async fn create_user_record(db: &mut Db, id: UserId) -> UserRecord {
        UserRecord::create()
            .id(RecordUserId::from(id))
            .name(Name(EN).fake::<String>())
            .exec(db)
            .await
            .unwrap()
    }

    async fn create_relationship_record(
        db: &mut Db,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Uuid {
        toasty::create!(UserRelationshipRecord {
            id: Uuid::now_v7(),
            follower_id: Uuid::from(follower_id),
            followee_id: Uuid::from(followee_id),
        })
        .exec(db)
        .await
        .unwrap()
        .id
    }

    #[tokio::test]
    async fn test_create_user_relationship_succeeds() {
        let _guard = DB_LOCK.lock().await;
        let (mut db, repository) = set_up_tests_db().await;
        let follower_id = UserId::new();
        let followee_id = UserId::new();
        create_user_record(&mut db, follower_id).await;
        create_user_record(&mut db, followee_id).await;
        let relationship = UserRelationship::new(follower_id, followee_id).unwrap();

        let created = repository
            .create_user_relationship(relationship)
            .await
            .unwrap();

        assert_eq!(created.follower_id, follower_id);
        assert_eq!(created.followee_id, followee_id);
        let records = Query::<List<UserRelationshipRecord>>::all()
            .exec(&mut db)
            .await
            .unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].follower_id, Uuid::from(follower_id));
        assert_eq!(records[0].followee_id, Uuid::from(followee_id));
        tear_down().await;
    }

    #[tokio::test]
    async fn test_create_user_relationship_fails_when_relationship_already_exists() {
        let _guard = DB_LOCK.lock().await;
        let (mut db, repository) = set_up_tests_db().await;
        let follower_id = UserId::new();
        let followee_id = UserId::new();
        create_user_record(&mut db, follower_id).await;
        create_user_record(&mut db, followee_id).await;
        let relationship = UserRelationship::new(follower_id, followee_id).unwrap();

        repository
            .create_user_relationship(relationship.clone())
            .await
            .unwrap();
        let duplicate = repository.create_user_relationship(relationship).await;

        assert!(matches!(duplicate, Err(RepositoryError::DatabaseError(_))));
        tear_down().await;
    }

    #[tokio::test]
    async fn test_get_user_relationship_succeeds() {
        let _guard = DB_LOCK.lock().await;
        let (mut db, repository) = set_up_tests_db().await;
        let follower_id = UserId::new();
        let followee_id = UserId::new();
        create_user_record(&mut db, follower_id).await;
        create_user_record(&mut db, followee_id).await;
        // an unrelated pair that must not be returned by the filter
        create_relationship_record(&mut db, UserId::new(), UserId::new()).await;
        let relationship_id = create_relationship_record(&mut db, follower_id, followee_id).await;

        let fetched = repository
            .get_user_relationship(follower_id, followee_id)
            .await
            .unwrap();

        assert_eq!(fetched.id, relationship_id);
        assert_eq!(fetched.follower_id, follower_id);
        assert_eq!(fetched.followee_id, followee_id);
        tear_down().await;
    }

    #[tokio::test]
    async fn test_get_user_relationship_fails_when_relationship_does_not_exist() {
        let _guard = DB_LOCK.lock().await;
        let (_db, repository) = set_up_tests_db().await;

        let fetched = repository
            .get_user_relationship(UserId::new(), UserId::new())
            .await;

        assert!(matches!(fetched, Err(RepositoryError::NotFound(_))));
        tear_down().await;
    }

    #[tokio::test]
    async fn test_list_user_relationships_by_follower_succeeds() {
        let _guard = DB_LOCK.lock().await;
        let (mut db, repository) = set_up_tests_db().await;
        let user_a = UserId::new();
        let user_b = UserId::new();
        let user_c = UserId::new();
        create_user_record(&mut db, user_a).await;
        create_user_record(&mut db, user_b).await;
        create_user_record(&mut db, user_c).await;
        let relationship_a_to_b = create_relationship_record(&mut db, user_a, user_b).await;
        let relationship_a_to_c = create_relationship_record(&mut db, user_a, user_c).await;
        // a is followee here, so it must not appear in the follower list
        create_relationship_record(&mut db, UserId::new(), user_a).await;

        let listed = repository
            .list_user_relationships_by_follower(user_a)
            .await
            .unwrap();

        assert_eq!(listed.len(), 2);
        assert!(listed.iter().any(|r| r.id == relationship_a_to_b));
        assert!(listed.iter().any(|r| r.id == relationship_a_to_c));
        tear_down().await;
    }

    #[tokio::test]
    async fn test_list_user_relationships_by_followee_succeeds() {
        let _guard = DB_LOCK.lock().await;
        let (mut db, repository) = set_up_tests_db().await;
        let user_a = UserId::new();
        let user_b = UserId::new();
        let user_c = UserId::new();
        create_user_record(&mut db, user_a).await;
        create_user_record(&mut db, user_b).await;
        create_user_record(&mut db, user_c).await;
        let relationship_a_to_b = create_relationship_record(&mut db, user_a, user_b).await;
        let relationship_c_to_b = create_relationship_record(&mut db, user_c, user_b).await;
        // b is follower here, so it must not appear in the followee list
        create_relationship_record(&mut db, user_b, UserId::new()).await;

        let listed = repository
            .list_user_relationships_by_followee(user_b)
            .await
            .unwrap();

        assert_eq!(listed.len(), 2);
        assert!(listed.iter().any(|r| r.id == relationship_a_to_b));
        assert!(listed.iter().any(|r| r.id == relationship_c_to_b));
        tear_down().await;
    }

    #[tokio::test]
    async fn test_list_user_relationships_by_follower_succeeds_with_empty() {
        let _guard = DB_LOCK.lock().await;
        let (_db, repository) = set_up_tests_db().await;

        let listed = repository
            .list_user_relationships_by_follower(UserId::new())
            .await
            .unwrap();

        assert!(listed.is_empty());
        tear_down().await;
    }

    #[tokio::test]
    async fn test_list_user_relationships_by_followee_succeeds_with_empty() {
        let _guard = DB_LOCK.lock().await;
        let (_db, repository) = set_up_tests_db().await;

        let listed = repository
            .list_user_relationships_by_followee(UserId::new())
            .await
            .unwrap();

        assert!(listed.is_empty());
        tear_down().await;
    }

    #[tokio::test]
    async fn test_delete_user_relationship_succeeds() {
        let _guard = DB_LOCK.lock().await;
        let (mut db, repository) = set_up_tests_db().await;
        let follower_id = UserId::new();
        let followee_id = UserId::new();
        create_user_record(&mut db, follower_id).await;
        create_user_record(&mut db, followee_id).await;
        create_relationship_record(&mut db, follower_id, followee_id).await;

        repository
            .delete_user_relationship(follower_id, followee_id)
            .await
            .unwrap();

        let record = UserRelationshipRecord::filter_by_follower_id_and_followee_id(
            Uuid::from(follower_id),
            Uuid::from(followee_id),
        )
        .first()
        .exec(&mut db)
        .await
        .unwrap();
        assert!(record.is_none());
        tear_down().await;
    }
}
