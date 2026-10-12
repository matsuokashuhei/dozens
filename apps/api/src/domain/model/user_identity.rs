use uuid::Uuid;

use crate::domain::model::user_id::UserId;

#[derive(Debug, Clone)]
pub struct UserIdentity {
    pub id: Uuid,
    pub user_id: UserId,
    pub iss: String,
    pub sub: String,
}

impl UserIdentity {
    pub fn new(user_id: UserId, iss: String, sub: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id,
            iss,
            sub,
        }
    }
}
