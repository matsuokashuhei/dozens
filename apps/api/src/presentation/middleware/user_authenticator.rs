use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json, RequestPartsExt};
use serde::Serialize;

use crate::{
    application::usecase::{
        authenticate_user_usecase::{AuthenticateUserInput, AuthenticateUserUsecase},
        errors::UsecaseError,
    },
    domain::model::user::User,
    presentation::middleware::token_authenticator::{TokenAuthenticator, TokenAuthenticatorError},
};

#[derive(Debug)]
pub struct UserAuthenticator {
    pub user: User,
    pub access_token: String,
}

#[derive(Debug)]
pub enum UserAuthenticatorError {
    Token(TokenAuthenticatorError),
    UserNotFound,
    Unavailable,
}

impl From<TokenAuthenticatorError> for UserAuthenticatorError {
    fn from(error: TokenAuthenticatorError) -> Self {
        Self::Token(error)
    }
}

impl From<UsecaseError> for UserAuthenticatorError {
    fn from(error: UsecaseError) -> Self {
        match error {
            UsecaseError::UserNotFound => Self::UserNotFound,
            _ => Self::Unavailable,
        }
    }
}

impl<S> FromRequestParts<S> for UserAuthenticator
where
    S: Send + Sync,
{
    type Rejection = UserAuthenticatorError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = parts.extract::<TokenAuthenticator>().await?;
        let (sub, access_token) = match (token.subject(), token.access_token()) {
            (Some(sub), Some(access_token)) => (sub.to_string(), access_token.to_string()),
            _ => return Err(TokenAuthenticatorError::Unauthorized.into()),
        };
        let Extension(authenticate_user_usecase) = parts
            .extract::<Extension<Arc<AuthenticateUserUsecase>>>()
            .await
            .map_err(|_| UserAuthenticatorError::Unavailable)?;
        let output = authenticate_user_usecase
            .execute(AuthenticateUserInput { sub })
            .await
            .map_err(UserAuthenticatorError::from)?;
        Ok(Self {
            user: output.user,
            access_token,
        })
    }
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
}

impl IntoResponse for UserAuthenticatorError {
    fn into_response(self) -> Response {
        match self {
            UserAuthenticatorError::Token(error) => error.into_response(),
            UserAuthenticatorError::UserNotFound => {
                (StatusCode::NOT_FOUND, Json(ErrorBody { code: "not_found" })).into_response()
            }
            UserAuthenticatorError::Unavailable => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody {
                    code: "unavailable",
                }),
            )
                .into_response(),
        }
    }
}
