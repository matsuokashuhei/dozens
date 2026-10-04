use std::env;

use aws_sdk_cognitoidentityprovider::{
    operation::admin_create_user::AdminCreateUserOutput, types::MessageActionType,
};

use crate::{
    domain::model::email::Email,
    infrastructure::service::cognito_identity_provider::CognitoIdentityProvider,
};

pub(crate) static TEST_EMAILS: [&str; 3] = [
    "matsuokashuheiii+test1@gmail.com",
    "matsuokashuheiii+test2@gmail.com",
    "matsuokashuheiii+test3@gmail.com",
];

pub(crate) async fn build_cognito_client() -> aws_sdk_cognitoidentityprovider::Client {
    let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;
    aws_sdk_cognitoidentityprovider::Client::new(&aws_config)
}

pub(crate) async fn build_cognito_identity_provider() -> CognitoIdentityProvider {
    let client = build_cognito_client().await;
    CognitoIdentityProvider::new(client)
}

pub(crate) async fn create_cognito_user(email: Email) -> AdminCreateUserOutput {
    let client = build_cognito_client().await;
    client
        .admin_create_user()
        .user_pool_id(env::var("AWS_COGNITO_USER_POOL_ID").unwrap())
        .username(email.as_str())
        .message_action(MessageActionType::Suppress)
        .send()
        .await
        .unwrap()
}

pub(crate) async fn delete_cognito_user(email: Email) {
    let client = build_cognito_client().await;
    let user_pool_id = env::var("AWS_COGNITO_USER_POOL_ID").unwrap();
    let output = client
        .admin_get_user()
        .user_pool_id(&user_pool_id)
        .username(email.as_str())
        .send()
        .await;
    if let Ok(output) = output {
        let username = output.username;
        client
            .admin_delete_user()
            .user_pool_id(&user_pool_id)
            .username(username.as_str())
            .send()
            .await
            .unwrap();
    }
}

pub(crate) async fn delete_cognito_users() {
    if env::var("GITHUB_ACTIONS").is_ok_and(|value| value == "true") {
        return;
    }
    for email in TEST_EMAILS {
        delete_cognito_user(Email::new(email).unwrap()).await;
    }
}
