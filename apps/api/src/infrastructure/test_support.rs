use std::{
    env,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use aws_sdk_cognitoidentityprovider::{
    operation::admin_create_user::AdminCreateUserOutput, types::MessageActionType,
};
use toasty::{
    Db,
    schema::Model,
    stmt::{List, Query},
};
use tracing::info;

use crate::{
    application::usecase::confirm_sign_up_usecase::ConfirmSignUpUsecase,
    domain::model::email::Email,
    infrastructure::{
        self,
        repository::{
            DatabaseError, user_identity_repository::UserIdentityRecord,
            user_repository::UserRecord,
        },
        service::cognito_identity_provider::CognitoIdentityProvider,
    },
    presentation::{
        handler::confirm_sign_up_handler::ConfirmSignUpHandler,
        router::confirm_sign_up_router::ConfirmSignUpRouter,
    },
};

pub static TEST_EMAILS: [&str; 3] = [
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

pub async fn build_db_connection() -> Result<Db, DatabaseError> {
    infrastructure::repository::build_db_connection(
        "postgresql://postgres:postgres@localhost:5433/dozens_test",
    )
    .await
}

pub async fn set_up() {
    tracing_subscriber::fmt::init();
    info!("Setting up test environment");
    tear_down().await;
    let db = build_db_connection().await.unwrap();
    toasty::embed_migrations!().apply(&db).await.unwrap();
}

pub async fn tear_down() {
    let mut db = build_db_connection().await.unwrap();
    delete_records::<UserIdentityRecord>(&mut db).await;
    delete_records::<UserRecord>(&mut db).await;
    delete_cognito_users().await;
}

async fn delete_records<T: Model>(db: &mut Db) {
    Query::<List<T>>::all().delete().exec(db).await.unwrap();
}

pub async fn create_cognito_user(email: Email) -> AdminCreateUserOutput {
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

pub async fn delete_cognito_user(email: Email) {
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

async fn delete_cognito_users() {
    for email in TEST_EMAILS {
        delete_cognito_user(Email::new(email).unwrap()).await;
    }
}

// pub async fn build_sign_up_usecase() -> SignUpUsecase {
//     let db = build_db_connection().await.unwrap();
//     let user_repository = UserRepositoryImpl::new(db.clone());
//     let user_identity_repository = UserIdentityRepositoryImpl::new(db.clone());
//     let identity_provider = build_cognito_identity_provider().await;
//     SignUpUsecase::new(
//         Arc::new(identity_provider),
//         Arc::new(user_repository),
//         Arc::new(user_identity_repository),
//     )
// }

// pub async fn build_sign_up_router() -> SignUpRouter {
//     let sign_up_usecase = build_sign_up_usecase().await;
//     let handler = SignUpHandler::new(Arc::new(sign_up_usecase));
//     SignUpRouter::new(Arc::new(handler))
// }

pub async fn build_confirm_sign_up_usecase() -> ConfirmSignUpUsecase {
    let identity_provider = build_cognito_identity_provider().await;
    ConfirmSignUpUsecase::new(Arc::new(identity_provider))
}

pub async fn build_confirm_sign_up_router() -> ConfirmSignUpRouter {
    let confirm_sign_up_usecase = build_confirm_sign_up_usecase().await;
    let handler = ConfirmSignUpHandler::new(Arc::new(confirm_sign_up_usecase));
    ConfirmSignUpRouter::new(Arc::new(handler))
}

pub async fn fetch_confirmation_code(username: &str, email: &str) -> String {
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
