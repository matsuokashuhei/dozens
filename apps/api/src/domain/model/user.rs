use serde::Serialize;

use crate::domain::model::user_id::UserId;

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

#[cfg(test)]
mod tests {
    use fake::{Fake, faker::name::raw::Name, locales::EN};
    use serde_json::json;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_user_new() {
        let name: String = Name(EN).fake();
        let user = User::new(name.clone());
        assert!(Uuid::parse_str(&user.id.to_string()).is_ok());
        assert_eq!(user.name, name);
    }

    #[test]
    fn test_user_json_keeps_id_as_string() {
        let user = User::new("ada".to_string());
        let serialized = serde_json::to_value(&user).unwrap();
        assert_eq!(
            serialized,
            json!({
                "id": user.id.to_string(),
                "name": "ada",
            })
        );
    }
}
