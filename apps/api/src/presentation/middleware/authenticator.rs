use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use axum_jwt_auth::{BearerTokenExtractor, Decoder, JwtDecoder, TokenExtractor};
use serde::Serialize;

use crate::application::service::identity_provider::Claims;

#[derive(Debug)]
pub struct Authenticator {
    subject: String,
}

impl Authenticator {
    pub fn subject(&self) -> &str {
        &self.subject
    }
}

#[derive(Debug)]
pub enum AuthenticatorError {
    Unauthorized,
    Unavailable,
}

pub fn extension(decoder: impl JwtDecoder<Claims> + 'static) -> Extension<Decoder<Claims>> {
    Extension(Arc::new(decoder))
}

impl<S> FromRequestParts<S> for Authenticator
where
    S: Send + Sync,
{
    type Rejection = AuthenticatorError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let decoder = parts
            .extensions
            .get::<Decoder<Claims>>()
            .cloned()
            .ok_or(AuthenticatorError::Unavailable)?;
        let token = BearerTokenExtractor::extract_token(parts)
            .await
            .map_err(|_| AuthenticatorError::Unauthorized)?;
        let data = decoder
            .decode(&token)
            .await
            .map_err(|_| AuthenticatorError::Unauthorized)?;
        Ok(Self {
            subject: data.claims.sub,
        })
    }
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
}

impl IntoResponse for AuthenticatorError {
    fn into_response(self) -> Response {
        let (status, code) = match self {
            AuthenticatorError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AuthenticatorError::Unavailable => (StatusCode::INTERNAL_SERVER_ERROR, "unavailable"),
        };
        (status, Json(ErrorBody { code })).into_response()
    }
}
