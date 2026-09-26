use thiserror::Error;

use crate::{
    application::service::identity_provider::IdentityProviderError,
    domain::repository::errors::RepositoryError,
};

#[derive(Debug, Error, PartialEq)]
pub enum UsecaseError {
    #[error("invalid parameter")]
    InvalidParameter,
    #[error("user not found")]
    UserNotFound,
    #[error("user not confirmed")]
    UserNotConfirmed,
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("code delivery failure")]
    CodeDeliveryFailure,
    #[error("code mismatch")]
    CodeMismatch,
    #[error("expired code")]
    ExpiredCode,
    #[error("unexpected error: {0}")]
    InternalError(String),
}

impl From<IdentityProviderError> for UsecaseError {
    fn from(error: IdentityProviderError) -> Self {
        match error {
            IdentityProviderError::InvalidParameter => Self::InvalidParameter,
            IdentityProviderError::UserNotFound => Self::UserNotFound,
            IdentityProviderError::UserNotConfirmed => Self::UserNotConfirmed,
            IdentityProviderError::UserAlreadyExists => Self::UserAlreadyExists,
            IdentityProviderError::CodeDeliveryFailure => Self::CodeDeliveryFailure,
            IdentityProviderError::CodeMismatch => Self::CodeMismatch,
            IdentityProviderError::ExpiredCode => Self::ExpiredCode,
            IdentityProviderError::InternalError { message } => Self::InternalError(message),
        }
    }
}

impl From<RepositoryError> for UsecaseError {
    fn from(error: RepositoryError) -> Self {
        Self::InternalError(error.to_string())
    }
}
