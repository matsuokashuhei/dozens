use async_trait::async_trait;
use thiserror::Error;

use crate::domain::model::email::Email;

#[derive(Debug)]
pub struct SignUpResult {
    pub iss: String,
    pub sub: String,
}

pub struct SignInResult {
    pub session: String,
}

#[derive(Debug)]
pub struct ConfirmSignUpResult {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
}

pub struct SendConfirmationCodeResult {}

#[derive(Debug)]
pub struct ConfirmSignInResult {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
}

#[derive(Debug, Error, PartialEq)]
pub enum IdentityProviderError {
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
    #[error("internal error: {message:?}")]
    InternalError { message: String },
}

#[async_trait]
pub trait IdentityProvider: Send + Sync {
    async fn sign_up(&self, email: Email) -> Result<SignUpResult, IdentityProviderError>;
    async fn confirm_sign_up(
        &self,
        email: Email,
        code: String,
    ) -> Result<ConfirmSignUpResult, IdentityProviderError>;
    async fn resend_confirmation_code(
        &self,
        email: Email,
    ) -> Result<SendConfirmationCodeResult, IdentityProviderError>;
    async fn sign_in(&self, email: Email) -> Result<SignInResult, IdentityProviderError>;
    async fn confirm_sign_in(
        &self,
        session: String,
        email: Email,
        code: String,
    ) -> Result<ConfirmSignInResult, IdentityProviderError>;
}
