use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, de::Error as _};
use validator::ValidateEmail;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Email(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidEmail;

impl fmt::Display for InvalidEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid email")
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Email::new(&raw).map_err(D::Error::custom)
    }
}

impl Email {
    pub fn new(raw: &str) -> Result<Self, InvalidEmail> {
        if !raw.validate_email() {
            return Err(InvalidEmail);
        }
        Ok(Self(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Email {
    type Error = InvalidEmail;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        Self::new(raw)
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_valid_email() {
        let email = Email::new("A@B.com");
        assert_eq!(email, Ok(Email("A@B.com".to_string())));
    }

    #[test]
    fn new_with_invalid_email() {
        let email = Email::new("invalid-email");
        assert_eq!(email, Err(InvalidEmail));
    }

    #[test]
    fn try_from_with_valid_email() {
        let email = Email::try_from("A@B.com");
        assert_eq!(email, Ok(Email("A@B.com".to_string())));
    }

    #[test]
    fn try_from_with_invalid_email() {
        let email = Email::try_from("invalid-email");
        assert_eq!(email, Err(InvalidEmail));
    }

    #[test]
    fn fmt() {
        let email = Email::new("ada@example.com").unwrap();
        assert_eq!(email.to_string(), "ada@example.com");
    }
}
