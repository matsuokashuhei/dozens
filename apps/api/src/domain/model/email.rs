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
    use fake::{Fake, faker::internet::raw::FreeEmail, locales::EN};

    use super::*;

    #[test]
    fn test_new_succeeds() {
        let raw: String = FreeEmail(EN).fake();
        let email = Email::new(&raw);
        assert_eq!(email, Ok(Email(raw)));
    }

    #[test]
    fn test_new_fails_when_email_is_invalid() {
        let email = Email::new("invalid-email");
        assert_eq!(email, Err(InvalidEmail));
    }

    #[test]
    fn test_try_from_succeeds() {
        let raw: String = FreeEmail(EN).fake();
        let email = Email::try_from(raw.as_str());
        assert_eq!(email, Ok(Email(raw)));
    }

    #[test]
    fn test_try_from_fails_when_email_is_invalid() {
        let email = Email::try_from("invalid-email");
        assert_eq!(email, Err(InvalidEmail));
    }

    #[test]
    fn test_fmt_succeeds() {
        let raw: String = FreeEmail(EN).fake();
        let email = Email::new(&raw).unwrap();
        assert_eq!(email.to_string(), raw);
    }
}
