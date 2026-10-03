use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json, RequestPartsExt};
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_jwt_auth::Decoder;
use serde::Serialize;

use crate::application::service::identity_provider::Claims;

#[derive(Debug)]
pub struct TokenAuthenticator {
    subject: Option<String>,
    access_token: Option<String>,
}

impl TokenAuthenticator {
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }
}

#[derive(Debug)]
pub enum TokenAuthenticatorError {
    Unauthorized,
    Unavailable,
}

pub fn extension(decoder: Decoder<Claims>) -> Extension<Decoder<Claims>> {
    Extension(decoder)
}

impl<S> FromRequestParts<S> for TokenAuthenticator
where
    S: Send + Sync,
{
    type Rejection = TokenAuthenticatorError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let bearer = match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
            Ok(TypedHeader(Authorization(bearer))) => bearer,
            Err(rejection) if rejection.is_missing() => {
                return Ok(Self {
                    subject: None,
                    access_token: None,
                });
            }
            Err(_) => return Err(TokenAuthenticatorError::Unauthorized),
        };
        let decoder = parts
            .extensions
            .get::<Decoder<Claims>>()
            .cloned()
            .ok_or(TokenAuthenticatorError::Unavailable)?;
        let data = decoder
            .decode(bearer.token())
            .await
            .map_err(|_| TokenAuthenticatorError::Unauthorized)?;
        Ok(Self {
            subject: Some(data.claims.sub),
            access_token: Some(bearer.token().to_string()),
        })
    }
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
}

impl IntoResponse for TokenAuthenticatorError {
    fn into_response(self) -> Response {
        let (status, code) = match self {
            TokenAuthenticatorError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            TokenAuthenticatorError::Unavailable => {
                (StatusCode::INTERNAL_SERVER_ERROR, "unavailable")
            }
        };
        (status, Json(ErrorBody { code })).into_response()
    }
}
