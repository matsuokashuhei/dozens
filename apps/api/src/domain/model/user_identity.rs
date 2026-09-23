use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserIdentity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub iss: String,
    pub sub: String,
}

impl UserIdentity {
    pub fn new(user_id: Uuid, iss: String, sub: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id,
            iss,
            sub,
        }
    }
}
