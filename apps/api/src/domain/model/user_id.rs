use std::fmt;

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, toasty::Embed)]
pub struct UserId(Uuid);

impl UserId {
    // No Default impl: the only zero value would be the nil UUID, meaningless as an id
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_new() {
        let user_id = UserId::new();
        assert!(Uuid::parse_str(&user_id.to_string()).is_ok());
    }

    #[test]
    fn test_user_id_as_uuid() {
        let id = Uuid::now_v7();
        let user_id = UserId::from(id);
        assert_eq!(user_id.as_uuid(), id);
    }

    #[test]
    fn test_user_id_display() {
        let id = Uuid::now_v7();
        assert_eq!(UserId::from(id).to_string(), id.to_string());
    }
}
