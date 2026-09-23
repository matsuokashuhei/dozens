use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, faker::name::raw::Name, locales::EN};

    use super::*;

    #[test]
    fn test_user_new() {
        let name: String = Name(EN).fake();
        let user = User::new(name.clone());
        assert!(Uuid::parse_str(&user.id.to_string()).is_ok());
        assert_eq!(user.name, name);
    }
}
