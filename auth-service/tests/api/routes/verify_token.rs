use crate::helpers::TestApp;
use authservice::constants::JWT_COOKIE_NAME;
use fake::Fake;
use fake::faker::internet::raw::SafeEmail;
use fake::locales::EN;
use reqwest::StatusCode;
use serde_json::{Value, json};

#[tokio::test]
async fn test_given_valid_token_when_post_verify_token_then_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let correct_email: String = SafeEmail(EN).fake();
    let correct_password = "Password123!";

    let _ = test_app
        .post_signup(&json!({
            "email":  correct_email,
            "password": correct_password,
            "requires2FA": false,
        }))
        .await;

    let login_response = test_app
        .post_login(&json!({
            "email":  correct_email,
            "password": correct_password,
        }))
        .await;

    let auth_cookie = login_response
        .cookies()
        .find(|c| c.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found.");

    let token = auth_cookie.value();

    let response = test_app.post_verify_token(&json!({ "token": token })).await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_given_invalid_token_when_post_verify_token_then_401() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let token = "fake-token";

    let response = test_app.post_verify_token(&json!({ "token": token })).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_given_malformed_payload_when_post_verify_token_then_401() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let test_cases: Vec<Value> = vec![
        json!({ "tokenn": "fake-token"}),
        json!({ "": "fake-token"}),
        json!({}),
        "".into(),
    ];

    for token in test_cases {
        let response = test_app.post_verify_token(&token).await;

        assert_eq!(
            response.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "Failed POST /verify-token with payload {token}"
        );
    }
}

#[tokio::test]
async fn test_given_revoked_token_when_post_verify_token_then_401() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let correct_email: String = SafeEmail(EN).fake();
    let correct_password = "Password123!";

    let _ = test_app
        .post_signup(&json!({
            "email":  correct_email,
            "password": correct_password,
            "requires2FA": false,
        }))
        .await;

    // Get token
    let login_response = test_app
        .post_login(&json!({
            "email":  correct_email,
            "password": correct_password,
        }))
        .await;

    let cookie = login_response
        .cookies()
        .find(|c| c.name() == JWT_COOKIE_NAME)
        .expect("failed to get find jwt cookie on login response");

    let token = cookie.value();

    let _ = test_app.post_logout().await;

    let response = test_app.post_verify_token(&json!({ "token": token })).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
