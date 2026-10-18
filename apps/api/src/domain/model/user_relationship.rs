use thiserror::Error;
use uuid::Uuid;

use crate::domain::model::user::UserId;

#[derive(Debug, Clone)]
pub struct UserRelationship {
    pub id: Uuid,
    pub follower_id: UserId,
    pub followee_id: UserId,
}

#[derive(Debug, Error, PartialEq)]
pub enum UserRelationshipError {
    #[error("self follow is not allowed")]
    SelfFollowNotAllowed,
}

impl UserRelationship {
    pub fn new(follower_id: UserId, followee_id: UserId) -> Result<Self, UserRelationshipError> {
        if follower_id == followee_id {
            return Err(UserRelationshipError::SelfFollowNotAllowed);
        }
        Ok(Self {
            id: Uuid::now_v7(),
            follower_id,
            followee_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_succeeds() {
        let follower_id = UserId::new();
        let followee_id = UserId::new();
        let relationship = UserRelationship::new(follower_id, followee_id).unwrap();
        assert!(Uuid::parse_str(&relationship.id.to_string()).is_ok());
        assert_eq!(relationship.follower_id, follower_id);
        assert_eq!(relationship.followee_id, followee_id);
    }

    #[test]
    fn test_new_fails_when_follower_equals_followee() {
        let user_id = UserId::new();
        assert!(matches!(
            UserRelationship::new(user_id, user_id),
            Err(UserRelationshipError::SelfFollowNotAllowed)
        ));
    }
}
