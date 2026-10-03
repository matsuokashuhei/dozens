use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use axum_jwt_auth::LocalDecoder;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, encode};
use serde::Serialize;

use crate::infrastructure::service::cognito_identity_provider::CognitoIdentityProvider;

// Development-only shared secret for HS256 tokens. Never used outside local auth mode.
const SECRET: &[u8] = b"dozens-local-auth-secret";
const DEFAULT_SUB: &str = "00000000-0000-0000-0000-000000000000";

pub fn issuer() -> String {
    CognitoIdentityProvider::issuer()
}

fn audience() -> String {
    env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap()
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
        .keys(vec![DecodingKey::from_secret(SECRET)])
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
        &EncodingKey::from_secret(SECRET),
    )
}
