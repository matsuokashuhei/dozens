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
    pub follower_id: UserId,
    #[index]
    pub followee_id: UserId,
    #[auto]
    created_at: Timestamp,
}

impl From<UserRelationshipRecord> for UserRelationship {
    fn from(record: UserRelationshipRecord) -> Self {
        UserRelationship {
            id: record.id,
            follower_id: record.follower_id,
            followee_id: record.followee_id,
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
            follower_id: user_relationship.follower_id,
            followee_id: user_relationship.followee_id,
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
            UserRelationshipRecord::filter_by_follower_id_and_followee_id(follower_id, followee_id)
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
        let records = UserRelationshipRecord::filter_by_follower_id(follower_id)
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
        let records = UserRelationshipRecord::filter_by_followee_id(followee_id)
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
            follower_id,
            followee_id,
        )
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, faker::name::raw::Name, locales::EN};

    use crate::{
        infrastructure::repository::user_repository::UserRecord,
        test_support::{self, set_up, tear_down},
    };

    use super::*;
    #[tokio::test]
    async fn test_user_relationship_crud() {
        set_up().await;
        let mut db = test_support::build_db_connection().await.unwrap();
        let repository = UserRelationshipRepositoryImpl::new(db.clone());
        let follower = UserRecord::create()
            .id(UserId::new())
            .name(Name(EN).fake::<String>())
            .exec(&mut db)
            .await
            .unwrap();
        let followee = UserRecord::create()
            .id(UserId::new())
            .name(Name(EN).fake::<String>())
            .exec(&mut db)
            .await
            .unwrap();
        let relationship = UserRelationship::new(follower.id, followee.id).unwrap();
        let relationship_id = relationship.id;

        let created = repository
            .create_user_relationship(relationship)
            .await
            .unwrap();
        assert_eq!(created.id, relationship_id);
        assert_eq!(created.follower_id, follower.id);
        assert_eq!(created.followee_id, followee.id);

        let fetched = repository
            .get_user_relationship(follower.id, followee.id)
            .await
            .unwrap();
        assert_eq!(fetched.id, relationship_id);

        let following = repository
            .list_user_relationships_by_follower(follower.id)
            .await
            .unwrap();
        assert_eq!(following.len(), 1);
        assert_eq!(following[0].id, relationship_id);

        let followers = repository
            .list_user_relationships_by_followee(followee.id)
            .await
            .unwrap();
        assert_eq!(followers.len(), 1);
        assert_eq!(followers[0].id, relationship_id);

        repository
            .delete_user_relationship(follower.id, followee.id)
            .await
            .unwrap();
        let missing = repository
            .get_user_relationship(follower.id, followee.id)
            .await;
        assert!(matches!(missing, Err(RepositoryError::NotFound(_))));
        tear_down().await;
    }

    #[tokio::test]
    async fn test_create_user_relationship_rejects_duplicate() {
        set_up().await;
        let mut db = test_support::build_db_connection().await.unwrap();
        let repository = UserRelationshipRepositoryImpl::new(db.clone());
        let follower = UserRecord::create()
            .id(UserId::new())
            .name(Name(EN).fake::<String>())
            .exec(&mut db)
            .await
            .unwrap();
        let followee = UserRecord::create()
            .id(UserId::new())
            .name(Name(EN).fake::<String>())
            .exec(&mut db)
            .await
            .unwrap();
        let relationship = UserRelationship::new(follower.id, followee.id).unwrap();

        repository
            .create_user_relationship(relationship.clone())
            .await
            .unwrap();
        let duplicate = repository.create_user_relationship(relationship).await;
        assert!(duplicate.is_err());
        tear_down().await;
    }
}
