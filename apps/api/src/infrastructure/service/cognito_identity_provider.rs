use std::env;
use std::future::Future;
use std::pin::Pin;

use async_trait::async_trait;
use aws_sdk_cognitoidentityprovider::{
    Client,
    operation::{
        admin_get_user::{AdminGetUserError, AdminGetUserOutput},
        confirm_sign_up::{ConfirmSignUpError, ConfirmSignUpOutput},
        global_sign_out::GlobalSignOutError,
        initiate_auth::{InitiateAuthError, InitiateAuthOutput},
        resend_confirmation_code::ResendConfirmationCodeError,
        respond_to_auth_challenge::{RespondToAuthChallengeError, RespondToAuthChallengeOutput},
        sign_up::{SignUpError, SignUpOutput},
        update_user_attributes::UpdateUserAttributesError,
        verify_user_attribute::VerifyUserAttributeError,
    },
    types::{AttributeType, AuthFlowType, ChallengeNameType, UserStatusType},
};
use axum_jwt_auth::{JwtDecoder, RemoteJwksDecoder};
use jsonwebtoken::{Algorithm, TokenData, Validation};
use tokio_util::sync::CancellationToken;
use tracing::error;

use crate::{
    application::service::identity_provider::{
        Claims, ConfirmSignInResult, ConfirmSignUpResult, IdentityProvider, IdentityProviderError,
        SendConfirmationCodeResult, SignInResult, SignUpResult,
    },
    domain::model::email::Email,
};

pub struct CognitoIdentityProvider {
    client: Client,
}

impl CognitoIdentityProvider {
    pub fn issuer() -> String {
        let region = env::var("AWS_REGION").unwrap();
        let user_pool_id = env::var("AWS_COGNITO_USER_POOL_ID").unwrap();
        format!("https://cognito-idp.{region}.amazonaws.com/{user_pool_id}")
    }

    pub fn new(client: Client) -> Self {
        Self { client }
    }

    async fn request_initiate_auth_with_session(
        &self,
        email: Email,
        session: String,
    ) -> Result<InitiateAuthOutput, IdentityProviderError> {
        self.client
            .initiate_auth()
            .client_id(env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap())
            .auth_flow(AuthFlowType::UserAuth)
            .session(session)
            .auth_parameters("USERNAME", email.as_str())
            .send()
            .await
            .map_err(|e| e.into_service_error().into())
    }

    pub(crate) async fn request_admin_get_user(
        &self,
        email: Email,
    ) -> Result<AdminGetUserOutput, IdentityProviderError> {
        self.client
            .admin_get_user()
            .user_pool_id(env::var("AWS_COGNITO_USER_POOL_ID").unwrap())
            .username(email.as_str())
            .send()
            .await
            .map_err(|e| e.into_service_error().into())
    }

    async fn request_sign_up(&self, email: Email) -> Result<SignUpOutput, IdentityProviderError> {
        self.client
            .sign_up()
            .client_id(env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap())
            .username(email.as_str())
            .send()
            .await
            .map_err(|e| e.into_service_error().into())
    }

    async fn request_confirm_sign_up(
        &self,
        email: Email,
        code: String,
    ) -> Result<ConfirmSignUpOutput, IdentityProviderError> {
        self.client
            .confirm_sign_up()
            .client_id(env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap())
            .username(email.as_str())
            .confirmation_code(code)
            .send()
            .await
            .map_err(|e| e.into_service_error().into())
    }

    async fn request_initiate_auth_with_email(
        &self,
        email: Email,
    ) -> Result<InitiateAuthOutput, IdentityProviderError> {
        self.client
            .initiate_auth()
            .client_id(env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap())
            .auth_flow(AuthFlowType::UserAuth)
            .auth_parameters("USERNAME", email.as_str())
            .auth_parameters("PREFERRED_CHALLENGE", "EMAIL_OTP")
            .send()
            .await
            .map_err(|e| e.into_service_error().into())
    }

    async fn request_respond_to_auth_challenge(
        &self,
        session: String,
        email: Email,
        code: String,
    ) -> Result<RespondToAuthChallengeOutput, IdentityProviderError> {
        self.client
            .respond_to_auth_challenge()
            .client_id(env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap())
            .challenge_name(ChallengeNameType::EmailOtp)
            .session(session)
            .challenge_responses("USERNAME", email.as_str())
            .challenge_responses("EMAIL_OTP_CODE", code)
            .send()
            .await
            .map_err(|e| e.into_service_error().into())
    }
}

#[async_trait]
impl IdentityProvider for CognitoIdentityProvider {
    async fn sign_up(&self, email: Email) -> Result<SignUpResult, IdentityProviderError> {
        let result = self.request_sign_up(email.clone()).await;
        match result {
            Ok(output) => Ok(SignUpResult {
                iss: Self::issuer(),
                sub: output.user_sub().to_owned(),
            }),
            Err(e) => match e {
                IdentityProviderError::UserAlreadyExists => {
                    let result = self.request_admin_get_user(email.clone()).await;
                    match result {
                        Ok(result) => {
                            if result.user_status() == Some(&UserStatusType::Unconfirmed) {
                                self.resend_confirmation_code(email).await?;
                                Ok(SignUpResult {
                                    iss: Self::issuer(),
                                    sub: result.username.to_owned(),
                                })
                            } else {
                                Err(IdentityProviderError::UserAlreadyExists)
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => Err(e),
            },
        }
    }

    async fn confirm_sign_up(
        &self,
        email: Email,
        code: String,
    ) -> Result<ConfirmSignUpResult, IdentityProviderError> {
        let result = self.request_confirm_sign_up(email.clone(), code).await;
        match result {
            Ok(output) => match output.session() {
                Some(session) => {
                    let output = self
                        .request_initiate_auth_with_session(email.clone(), session.to_owned())
                        .await?;
                    match output.authentication_result() {
                        Some(authentication_result) => {
                            let (Some(access_token), Some(refresh_token), Some(id_token)) = (
                                authentication_result.access_token(),
                                authentication_result.refresh_token(),
                                authentication_result.id_token(),
                            ) else {
                                return Err(IdentityProviderError::InternalError {
                                    message: "No authentication result found".to_string(),
                                });
                            };
                            Ok(ConfirmSignUpResult {
                                access_token: access_token.to_owned(),
                                refresh_token: refresh_token.to_owned(),
                                id_token: id_token.to_owned(),
                            })
                        }
                        None => Err(IdentityProviderError::InternalError {
                            message: "No authentication result found".to_string(),
                        }),
                    }
                }
                None => Err(IdentityProviderError::InternalError {
                    message: "No session found".to_string(),
                }),
            },
            Err(e) => {
                error!("Failed to confirm sign up: {:?}", e);
                Err(e)
            }
        }
    }

    async fn resend_confirmation_code(
        &self,
        email: Email,
    ) -> Result<SendConfirmationCodeResult, IdentityProviderError> {
        let result = self
            .client
            .resend_confirmation_code()
            .client_id(env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap())
            .username(email.as_str())
            .send()
            .await;
        match result {
            Ok(_) => Ok(SendConfirmationCodeResult {}),
            Err(e) => Err(e.into_service_error().into()),
        }
    }

    async fn sign_in(&self, email: Email) -> Result<SignInResult, IdentityProviderError> {
        let result = self.request_initiate_auth_with_email(email.clone()).await;
        match result {
            Ok(output) => match output.session() {
                Some(session) => Ok(SignInResult {
                    session: session.to_owned(),
                }),
                None => Err(IdentityProviderError::InternalError {
                    message: "No session found".to_string(),
                }),
            },
            Err(e) => Err(e),
        }
    }

    async fn confirm_sign_in(
        &self,
        session: String,
        email: Email,
        code: String,
    ) -> Result<ConfirmSignInResult, IdentityProviderError> {
        let result = self
            .request_respond_to_auth_challenge(session, email, code)
            .await;
        match result {
            Ok(output) => match output.authentication_result() {
                Some(authentication_result) => {
                    let (Some(access_token), Some(refresh_token), Some(id_token)) = (
                        authentication_result.access_token(),
                        authentication_result.refresh_token(),
                        authentication_result.id_token(),
                    ) else {
                        error!("No authentication result found");
                        return Err(IdentityProviderError::InternalError {
                            message: "No authentication result found".to_string(),
                        });
                    };
                    Ok(ConfirmSignInResult {
                        access_token: access_token.to_owned(),
                        refresh_token: refresh_token.to_owned(),
                        id_token: id_token.to_owned(),
                    })
                }
                None => {
                    error!("No authentication result found");
                    Err(IdentityProviderError::InternalError {
                        message: "No session found".to_string(),
                    })
                }
            },
            Err(e) => {
                error!("Failed to confirm sign in: {:?}", e);
                Err(e)
            }
        }
    }

    async fn sign_out(&self, access_token: String) -> Result<(), IdentityProviderError> {
        self.client
            .global_sign_out()
            .access_token(access_token)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| e.into_service_error().into())
    }

    async fn change_email(
        &self,
        access_token: String,
        email: Email,
    ) -> Result<(), IdentityProviderError> {
        self.client
            .update_user_attributes()
            .access_token(access_token)
            .user_attributes(
                AttributeType::builder()
                    .name("email")
                    .value(email.as_str())
                    .build()
                    .map_err(|e| IdentityProviderError::InternalError {
                        message: e.to_string(),
                    })?,
            )
            .send()
            .await
            .map(|_| ())
            .map_err(|e| e.into_service_error().into())
    }

    async fn confirm_change_email(
        &self,
        access_token: String,
        code: String,
    ) -> Result<(), IdentityProviderError> {
        self.client
            .verify_user_attribute()
            .access_token(access_token)
            .attribute_name("email")
            .code(code)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| e.into_service_error().into())
    }

    async fn build_token_decoder(
        &self,
    ) -> Result<impl JwtDecoder<Claims> + 'static, IdentityProviderError> {
        let issuer = Self::issuer();
        let jwks_url = format!("{issuer}/.well-known/jwks.json");
        let mut validation = Validation::new(Algorithm::RS256);
        // Cognito access tokens omit `aud`.
        validation.validate_aud = false;
        validation.set_issuer(&[&issuer]);
        validation.set_required_spec_claims(&["exp", "iss", "sub"]);
        let inner = RemoteJwksDecoder::builder()
            .jwks_url(jwks_url)
            .validation(validation)
            .build()
            .map_err(|e| IdentityProviderError::InternalError {
                message: e.to_string(),
            })?;
        let shutdown =
            inner
                .initialize()
                .await
                .map_err(|e| IdentityProviderError::InternalError {
                    message: e.to_string(),
                })?;
        Ok(CognitoAccessTokenDecoder { inner, shutdown })
    }
}

struct CognitoAccessTokenDecoder {
    inner: RemoteJwksDecoder,
    // Hourly JWKS refresh rotates keys until this token is cancelled.
    shutdown: CancellationToken,
}

impl Drop for CognitoAccessTokenDecoder {
    fn drop(&mut self) {
        self.shutdown.cancel();
    }
}

impl JwtDecoder<Claims> for CognitoAccessTokenDecoder {
    fn decode<'a>(
        &'a self,
        token: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<TokenData<Claims>, axum_jwt_auth::Error>> + Send + 'a>>
    {
        JwtDecoder::<Claims>::decode(&self.inner, token)
    }
}

impl From<SignUpError> for IdentityProviderError {
    fn from(e: SignUpError) -> Self {
        match e {
            SignUpError::InvalidParameterException(_) => IdentityProviderError::InvalidParameter,
            SignUpError::UsernameExistsException(_) => IdentityProviderError::UserAlreadyExists,
            SignUpError::CodeDeliveryFailureException(_) => {
                IdentityProviderError::CodeDeliveryFailure
            }
            _ => IdentityProviderError::InternalError {
                message: e.to_string(),
            },
        }
    }
}

impl From<InitiateAuthError> for IdentityProviderError {
    fn from(e: InitiateAuthError) -> Self {
        match e {
            InitiateAuthError::InvalidParameterException(_) => Self::InvalidParameter,
            InitiateAuthError::UserNotFoundException(_) => Self::UserNotFound,
            InitiateAuthError::UserNotConfirmedException(_) => Self::UserNotConfirmed,
            _ => Self::InternalError {
                message: e.to_string(),
            },
        }
    }
}

impl From<ConfirmSignUpError> for IdentityProviderError {
    fn from(e: ConfirmSignUpError) -> Self {
        match e {
            ConfirmSignUpError::InvalidParameterException(_) => Self::InvalidParameter,
            ConfirmSignUpError::CodeMismatchException(_) => Self::CodeMismatch,
            ConfirmSignUpError::ExpiredCodeException(_) => Self::ExpiredCode,
            ConfirmSignUpError::UserNotFoundException(_) => Self::UserNotFound,
            _ => Self::InternalError {
                message: e.to_string(),
            },
        }
    }
}

impl From<ResendConfirmationCodeError> for IdentityProviderError {
    fn from(e: ResendConfirmationCodeError) -> Self {
        match e {
            ResendConfirmationCodeError::InvalidParameterException(_) => Self::InvalidParameter,
            ResendConfirmationCodeError::CodeDeliveryFailureException(_) => {
                Self::CodeDeliveryFailure
            }
            ResendConfirmationCodeError::UserNotFoundException(_) => Self::UserNotFound,
            _ => Self::InternalError {
                message: e.to_string(),
            },
        }
    }
}

impl From<RespondToAuthChallengeError> for IdentityProviderError {
    fn from(e: RespondToAuthChallengeError) -> Self {
        match e {
            RespondToAuthChallengeError::InvalidParameterException(_) => Self::InvalidParameter,
            RespondToAuthChallengeError::CodeMismatchException(_) => Self::CodeMismatch,
            RespondToAuthChallengeError::ExpiredCodeException(_) => Self::ExpiredCode,
            RespondToAuthChallengeError::UserNotFoundException(_) => Self::UserNotFound,
            RespondToAuthChallengeError::UserNotConfirmedException(_) => Self::UserNotConfirmed,
            _ => Self::InternalError {
                message: e.to_string(),
            },
        }
    }
}

impl From<GlobalSignOutError> for IdentityProviderError {
    fn from(e: GlobalSignOutError) -> Self {
        match e {
            GlobalSignOutError::InvalidParameterException(_) => Self::InvalidParameter,
            _ => Self::InternalError {
                message: e.to_string(),
            },
        }
    }
}

impl From<UpdateUserAttributesError> for IdentityProviderError {
    fn from(e: UpdateUserAttributesError) -> Self {
        match e {
            UpdateUserAttributesError::InvalidParameterException(_) => Self::InvalidParameter,
            UpdateUserAttributesError::CodeDeliveryFailureException(_) => Self::CodeDeliveryFailure,
            UpdateUserAttributesError::NotAuthorizedException(_) => Self::NotAuthorized,
            UpdateUserAttributesError::UserNotFoundException(_) => Self::UserNotFound,
            UpdateUserAttributesError::UserNotConfirmedException(_) => Self::UserNotConfirmed,
            UpdateUserAttributesError::AliasExistsException(_) => Self::UserAlreadyExists,
            _ => Self::InternalError {
                message: e.to_string(),
            },
        }
    }
}

impl From<VerifyUserAttributeError> for IdentityProviderError {
    fn from(e: VerifyUserAttributeError) -> Self {
        match e {
            VerifyUserAttributeError::InvalidParameterException(_) => Self::InvalidParameter,
            VerifyUserAttributeError::CodeMismatchException(_) => Self::CodeMismatch,
            VerifyUserAttributeError::ExpiredCodeException(_) => Self::ExpiredCode,
            VerifyUserAttributeError::NotAuthorizedException(_) => Self::NotAuthorized,
            VerifyUserAttributeError::UserNotFoundException(_) => Self::UserNotFound,
            VerifyUserAttributeError::UserNotConfirmedException(_) => Self::UserNotConfirmed,
            VerifyUserAttributeError::AliasExistsException(_) => Self::UserAlreadyExists,
            _ => Self::InternalError {
                message: e.to_string(),
            },
        }
    }
}

impl From<AdminGetUserError> for IdentityProviderError {
    fn from(e: AdminGetUserError) -> Self {
        match e {
            AdminGetUserError::InvalidParameterException(_) => Self::InvalidParameter,
            AdminGetUserError::UserNotFoundException(_) => Self::UserNotFound,
            _ => Self::InternalError {
                message: e.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, faker::internet::raw::FreeEmail, locales::EN};

    use crate::infrastructure::test_support::{
        TEST_EMAILS, build_cognito_identity_provider, create_cognito_user, fetch_confirmation_code,
        set_up, tear_down,
    };

    use super::*;

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_sign_up_with_success() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[0]).unwrap();
        let result = identity_provider.request_sign_up(email).await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_sign_up_with_user_already_exists() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[0]).unwrap();
        let result = identity_provider.request_sign_up(email).await;
        assert!(result.is_ok());
        let email = Email::new(TEST_EMAILS[0]).unwrap();
        let result = identity_provider.request_sign_up(email).await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            IdentityProviderError::UserAlreadyExists
        );
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_confirm_sign_up_with_success() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[1]).unwrap();
        let output = identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let confirmation_code =
            fetch_confirmation_code(output.user_sub(), email.clone().as_str()).await;
        let result = identity_provider
            .request_confirm_sign_up(email.clone(), confirmation_code.clone())
            .await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_confirm_sign_up_with_code_mismatch() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[1]).unwrap();
        identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let result = identity_provider
            .request_confirm_sign_up(email.clone(), "123456".to_string())
            .await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::CodeMismatch);
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_confirm_sign_up_with_user_not_found() {
        let identity_provider = build_cognito_identity_provider().await;
        let email: String = FreeEmail(EN).fake();
        let result = identity_provider
            .request_confirm_sign_up(Email::new(&email).unwrap(), "123456".to_string())
            .await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::UserNotFound);
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_sign_in_with_success() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[2]).unwrap();
        let output = identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let confirmation_code =
            fetch_confirmation_code(output.user_sub(), email.clone().as_str()).await;
        identity_provider
            .request_confirm_sign_up(email.clone(), confirmation_code.clone())
            .await
            .unwrap();
        let result = identity_provider
            .request_initiate_auth_with_email(email)
            .await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_sign_in_with_user_not_found() {
        let identity_provider = build_cognito_identity_provider().await;
        let email: String = FreeEmail(EN).fake();
        let result = identity_provider
            .request_initiate_auth_with_email(Email::new(&email).unwrap())
            .await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::UserNotFound);
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_sign_in_with_user_not_confirmed() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[2]).unwrap();
        identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let result = identity_provider
            .request_initiate_auth_with_email(email)
            .await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            IdentityProviderError::UserNotConfirmed
        );
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_send_confirmation_code_with_success() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[0]).unwrap();
        identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let result = identity_provider
            .resend_confirmation_code(email.clone())
            .await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_send_confirmation_code_with_user_not_found() {
        let identity_provider = build_cognito_identity_provider().await;
        let email: String = FreeEmail(EN).fake();
        let result = identity_provider
            .resend_confirmation_code(Email::new(&email).unwrap())
            .await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::UserNotFound);
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_respond_to_auth_challenge_with_success() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[1]).unwrap();
        let user_type = create_cognito_user(email.clone()).await.user.unwrap();
        let result = identity_provider.sign_in(email.clone()).await.unwrap();
        let confirmation_code =
            fetch_confirmation_code(user_type.username().unwrap(), email.clone().as_str()).await;
        let result = identity_provider
            .request_respond_to_auth_challenge(
                result.session,
                email.clone(),
                confirmation_code.clone(),
            )
            .await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_respond_to_auth_challenge_with_code_mismatch() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[1]).unwrap();
        create_cognito_user(email.clone()).await;
        let result = identity_provider.sign_in(email.clone()).await.unwrap();
        let result = identity_provider
            .request_respond_to_auth_challenge(result.session, email.clone(), "123456".to_string())
            .await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::CodeMismatch);
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_respond_to_auth_challenge_with_user_not_found() {
        let identity_provider = build_cognito_identity_provider().await;
        let email: String = FreeEmail(EN).fake();
        let result = identity_provider
            .request_respond_to_auth_challenge(
                "".to_string(),
                Email::new(&email).unwrap(),
                "123456".to_string(),
            )
            .await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::UserNotFound);
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_respond_to_auth_challenge_with_user_not_confirmed() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[1]).unwrap();
        let result = identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let result = identity_provider
            .request_respond_to_auth_challenge(
                result.session.unwrap(),
                email.clone(),
                "123456".to_string(),
            )
            .await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            IdentityProviderError::UserNotConfirmed
        );
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_admin_get_user_with_success() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[0]).unwrap();
        identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let result = identity_provider.request_admin_get_user(email).await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_request_admin_get_user_with_user_not_found() {
        let identity_provider = build_cognito_identity_provider().await;
        let email: String = FreeEmail(EN).fake();
        let result = identity_provider
            .request_admin_get_user(Email::new(&email).unwrap())
            .await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::UserNotFound);
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_sign_up_with_success() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[0]).unwrap();
        let result = identity_provider.sign_up(email).await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_sign_up_with_user_already_exists() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[0]).unwrap();
        let output = identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let confirmation_code =
            fetch_confirmation_code(output.user_sub(), email.clone().as_str()).await;
        identity_provider
            .request_confirm_sign_up(email.clone(), confirmation_code.clone())
            .await
            .unwrap();
        let result = identity_provider.sign_up(email).await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_sign_up_with_user_not_confirmed() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[0]).unwrap();
        identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let result = identity_provider.sign_up(email).await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_confirm_sign_up() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[1]).unwrap();
        let output = identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let confirmation_code =
            fetch_confirmation_code(output.user_sub(), email.clone().as_str()).await;
        let result = identity_provider
            .confirm_sign_up(email.clone(), confirmation_code.clone())
            .await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_confirm_sign_up_with_code_mismatch() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[1]).unwrap();
        identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let result = identity_provider
            .confirm_sign_up(email.clone(), "123456".to_string())
            .await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::CodeMismatch);
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_confirm_sign_up_with_user_not_found() {
        let identity_provider = build_cognito_identity_provider().await;
        let email: String = FreeEmail(EN).fake();
        let result = identity_provider
            .confirm_sign_up(Email::new(&email).unwrap(), "123456".to_string())
            .await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::UserNotFound);
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_sign_in() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[2]).unwrap();
        create_cognito_user(email.clone()).await;

        let result = identity_provider.sign_in(email).await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_sign_in_with_user_not_found() {
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[2]).unwrap();
        let result = identity_provider.sign_in(email).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::UserNotFound);
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_sign_in_with_user_not_confirmed() {
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[2]).unwrap();
        identity_provider
            .request_sign_up(email.clone())
            .await
            .unwrap();
        let result = identity_provider.sign_in(email).await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            IdentityProviderError::UserNotConfirmed
        );
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_confirm_sign_in() {
        set_up().await;
        let email = Email::new(TEST_EMAILS[1]).unwrap();
        let output = create_cognito_user(email.clone()).await.user.unwrap();
        let identity_provider = build_cognito_identity_provider().await;
        let result = identity_provider
            .request_initiate_auth_with_email(email.clone())
            .await
            .unwrap();
        let confirmation_code =
            fetch_confirmation_code(output.username().unwrap(), email.clone().as_str()).await;
        let result = identity_provider
            .confirm_sign_in(
                result.session.unwrap(),
                email.clone(),
                confirmation_code.clone(),
            )
            .await;
        assert!(result.is_ok());
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_confirm_sign_in_with_code_mismatch() {
        set_up().await;
        let email = Email::new(TEST_EMAILS[1]).unwrap();
        create_cognito_user(email.clone()).await;
        let identity_provider = build_cognito_identity_provider().await;
        let result = identity_provider
            .request_initiate_auth_with_email(email.clone())
            .await
            .unwrap();
        let result = identity_provider
            .confirm_sign_in(result.session.unwrap(), email.clone(), "123456".to_string())
            .await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::CodeMismatch);
        tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_confirm_sign_in_with_user_not_found() {
        set_up().await;
        let email = Email::new(TEST_EMAILS[1]).unwrap();
        create_cognito_user(email.clone()).await;
        let identity_provider = build_cognito_identity_provider().await;
        let result = identity_provider
            .request_initiate_auth_with_email(email.clone())
            .await
            .unwrap();
        let email: String = FreeEmail(EN).fake();
        let result = identity_provider
            .confirm_sign_in(
                result.session.unwrap(),
                Email::new(&email).unwrap(),
                "123456".to_string(),
            )
            .await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), IdentityProviderError::UserNotFound);
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_confirm_sign_in_with_user_not_confirmed() {
        // set_up().await;
        // let email = Email::new(TEST_EMAILS[1]).unwrap();
        // let identity_provider = build_cognito_identity_provider().await;
        // let output = identity_provider
        //     .request_sign_up(email.clone())
        //     .await
        //     .unwrap();
        // let result = identity_provider
        //     .confirm_sign_in(output.session.unwrap(), email.clone(), "123456".to_string())
        //     .await;
        // assert!(result.is_err());
        // assert_eq!(
        //     result.err().unwrap(),
        //     IdentityProviderError::UserNotConfirmed
        // );
        // tear_down().await;
    }

    #[tokio::test]
    #[ignore = "requires local AWS Cognito"]
    async fn test_sign_out() {
        set_up().await;
        let identity_provider = build_cognito_identity_provider().await;
        let email = Email::new(TEST_EMAILS[2]).unwrap();
        let output = create_cognito_user(email.clone()).await.user.unwrap();
        let result = identity_provider.sign_in(email.clone()).await.unwrap();
        let confirmation_code =
            fetch_confirmation_code(output.username().unwrap(), email.clone().as_str()).await;
        let result = identity_provider
            .confirm_sign_in(result.session, email.clone(), confirmation_code.clone())
            .await
            .unwrap();
        let result = identity_provider.sign_out(result.access_token).await;
        assert!(result.is_ok());
        tear_down().await;
    }
}
