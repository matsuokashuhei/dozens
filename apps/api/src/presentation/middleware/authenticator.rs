use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json, RequestPartsExt};
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_jwt_auth::{Decoder, JwtDecoder};
use serde::Serialize;

use crate::application::service::identity_provider::Claims;

#[derive(Debug)]
pub struct Authenticator {
    subject: Option<String>,
}

impl Authenticator {
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
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
        let bearer = match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
            Ok(TypedHeader(Authorization(bearer))) => bearer,
            Err(rejection) if rejection.is_missing() => return Ok(Self { subject: None }),
            Err(_) => return Err(AuthenticatorError::Unauthorized),
        };
        let decoder = parts
            .extensions
            .get::<Decoder<Claims>>()
            .cloned()
            .ok_or(AuthenticatorError::Unavailable)?;
        let data = decoder
            .decode(bearer.token())
            .await
            .map_err(|_| AuthenticatorError::Unauthorized)?;
        Ok(Self {
            subject: Some(data.claims.sub),
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
