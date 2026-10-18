use std::fmt;

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: UserId::new(),
            name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct UserId(Uuid);

impl UserId {
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
    use fake::{Fake, faker::name::raw::Name, locales::EN};
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_new_succeeds() {
        let name: String = Name(EN).fake();
        let user = User::new(name.clone());
        assert!(Uuid::parse_str(&user.id.to_string()).is_ok());
        assert_eq!(user.name, name);
    }
}
