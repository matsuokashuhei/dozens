use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use axum_jwt_auth::LocalDecoder;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, encode};
use serde::Serialize;

const DEFAULT_ISSUER: &str = "dozens-local";
const DEFAULT_AUDIENCE: &str = "dozens-local";
const DEFAULT_SUB: &str = "00000000-0000-0000-0000-000000000000";

pub fn secret() -> String {
    env::var("LOCAL_AUTH_SECRET").expect("LOCAL_AUTH_SECRET is required in local auth mode")
}

pub fn issuer() -> String {
    env::var("LOCAL_AUTH_ISSUER").unwrap_or_else(|_| DEFAULT_ISSUER.to_string())
}

pub fn audience() -> String {
    env::var("LOCAL_AUTH_AUDIENCE").unwrap_or_else(|_| DEFAULT_AUDIENCE.to_string())
}

pub fn subject() -> String {
    env::var("LOCAL_AUTH_SUB").unwrap_or_else(|_| DEFAULT_SUB.to_string())
}

pub fn build_decoder() -> Result<LocalDecoder, axum_jwt_auth::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[issuer()]);
    validation.set_audience(&[audience()]);
    validation.set_required_spec_claims(&["exp", "iss", "sub", "aud"]);
    LocalDecoder::builder()
        .keys(vec![DecodingKey::from_secret(secret().as_bytes())])
        .validation(validation)
        .build()
}

#[derive(Serialize)]
struct LocalClaims<'a> {
    sub: &'a str,
    iss: &'a str,
    aud: &'a str,
    exp: u64,
    token_use: &'a str,
}

pub fn issue_access_token(
    sub: &str,
    expires_in_secs: u64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_secs();
    let claims = LocalClaims {
        sub,
        iss: &issuer(),
        aud: &audience(),
        exp: now + expires_in_secs,
        token_use: "access",
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret().as_bytes()),
    )
}
