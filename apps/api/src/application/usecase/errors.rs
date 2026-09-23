use thiserror::Error;

use crate::domain::repository::errors::RepositoryError;

#[derive(Debug, Error)]
pub enum UsecaseError {
    #[error("unexpected error: {0}")]
    UnexpectedError(String),
}

impl From<RepositoryError> for UsecaseError {
    fn from(error: RepositoryError) -> Self {
        UsecaseError::UnexpectedError(error.to_string())
    }
}
