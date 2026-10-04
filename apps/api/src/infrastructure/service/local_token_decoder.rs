use std::future::Future;
use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};

use axum_jwt_auth::JwtDecoder;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, encode};

use crate::{
    application::service::identity_provider::Claims,
    infrastructure::service::cognito_identity_provider::CognitoIdentityProvider,
};

// Development-only shared secret for HS256 tokens. Never used outside local auth mode.
const SECRET: &[u8] = b"dozens-local-auth-secret";

pub fn issuer() -> String {
    CognitoIdentityProvider::issuer()
}

fn validation() -> Validation {
    let mut validation = Validation::new(Algorithm::HS256);
    // Cognito access tokens omit `aud`.
    validation.validate_aud = false;
    validation.set_issuer(&[issuer()]);
    validation.set_required_spec_claims(&["exp", "iss", "sub"]);
    validation
}

pub fn issue_access_token(
    sub: &str,
    expires_in_secs: u64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_secs();
    let claims = Claims {
        sub: sub.to_string(),
        exp: now + expires_in_secs,
        iss: issuer(),
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
}

pub struct LocalTokenDecoder {
    key: DecodingKey,
    validation: Validation,
}

impl LocalTokenDecoder {
    pub fn new() -> Self {
        Self {
            key: DecodingKey::from_secret(SECRET),
            validation: validation(),
        }
    }
}

impl Default for LocalTokenDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl JwtDecoder<Claims> for LocalTokenDecoder {
    fn decode<'a>(
        &'a self,
        token: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<TokenData<Claims>, axum_jwt_auth::Error>> + Send + 'a>>
    {
        Box::pin(async move {
            jsonwebtoken::decode::<Claims>(token, &self.key, &self.validation)
                .map_err(axum_jwt_auth::Error::Jwt)
        })
    }
}
