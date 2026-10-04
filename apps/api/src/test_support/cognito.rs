use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

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

pub(crate) async fn fetch_confirmation_code(username: &str, email: &str) -> String {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;
    let client = aws_sdk_cloudwatchlogs::Client::new(&config);
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
        - 10 * 60 * 1000;

    for _ in 0..30 {
        let mut next_token = None;
        let mut code = None;
        loop {
            let mut request = client
                .filter_log_events()
                .log_group_name("/aws/lambda/dozens-local-custom-email-sender")
                .filter_pattern(format!(
                    r#"{{$.type="cognito.otp" && $.email="{email}" && $.userName="{username}"}}"#
                ))
                .start_time(start_time);
            if let Some(token) = &next_token {
                request = request.next_token(token);
            }
            let page = request.send().await.unwrap();
            for event in page.events() {
                let Some(message) = event.message() else {
                    continue;
                };
                let Some(start) = message.find('{') else {
                    continue;
                };
                let Some(end) = message.rfind('}') else {
                    continue;
                };
                let Ok(value) = serde_json::from_str::<serde_json::Value>(&message[start..=end])
                else {
                    continue;
                };
                if let Some(found) = value.get("code").and_then(|code| code.as_str()) {
                    code = Some(found.to_owned());
                }
            }
            match page.next_token() {
                Some(token) => next_token = Some(token.to_owned()),
                None => break,
            }
        }
        if let Some(code) = code {
            return code;
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    panic!("confirmation code not found for {email}");
}
