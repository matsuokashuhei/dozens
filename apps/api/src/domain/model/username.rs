use fake::{Fake, faker::lorem::raw::Word, locales::EN};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Username(String);

impl Username {
    pub fn new(raw: &str) -> Self {
        Self(raw.to_string())
    }

    pub fn generate() -> Self {
        Self(Word(EN).fake())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Username {
    fn from(raw: String) -> Self {
        Self(raw)
    }
}
