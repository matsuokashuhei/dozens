use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
