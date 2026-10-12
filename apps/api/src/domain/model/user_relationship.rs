use std::fmt;

use uuid::Uuid;

use crate::domain::model::user_id::UserId;

#[derive(Debug, Clone)]
pub struct UserRelationship {
    pub id: Uuid,
    pub follower_id: UserId,
    pub followee_id: UserId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfFollow;

impl fmt::Display for SelfFollow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("self follow is not allowed")
    }
}

impl UserRelationship {
    pub fn new(follower_id: UserId, followee_id: UserId) -> Result<Self, SelfFollow> {
        if follower_id == followee_id {
            return Err(SelfFollow);
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
    fn test_user_relationship_new() {
        let follower_id = UserId::generate();
        let followee_id = UserId::generate();
        let relationship = UserRelationship::new(follower_id, followee_id).unwrap();
        assert!(Uuid::parse_str(&relationship.id.to_string()).is_ok());
        assert_eq!(relationship.follower_id, follower_id);
        assert_eq!(relationship.followee_id, followee_id);
    }

    #[test]
    fn test_user_relationship_new_rejects_self_follow() {
        let user_id = UserId::generate();
        assert!(matches!(
            UserRelationship::new(user_id, user_id),
            Err(SelfFollow)
        ));
    }
}
