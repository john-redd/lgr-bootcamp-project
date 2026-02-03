use crate::helpers::TestApp;
use authservice::constants::JWT_COOKIE_NAME;
use fake::Fake;
use fake::faker::internet::raw::SafeEmail;
use fake::locales::EN;
use reqwest::{StatusCode, Url};
use serde_json::json;

#[tokio::test]
async fn test_logout_endpoint_returns_200() {
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

    let login_response_cookie = login_response
        .cookies()
        .find(|c| c.name() == JWT_COOKIE_NAME)
        .expect("failed to get login cookie");
    let login_response_token = login_response_cookie.value();

    let response = test_app.post_logout().await;

    assert_eq!(response.status(), StatusCode::OK);

    let cookie = response
        .cookies()
        .find(|c| c.name() == JWT_COOKIE_NAME)
        .unwrap();

    assert!(cookie.value().is_empty());

    let banned_token_store_check = test_app
        .check_banned_token_store(login_response_token)
        .await;

    assert!(banned_token_store_check.is_some())
}

#[tokio::test]
async fn test_logout_endpoint_invoked_twice_returns_400() {
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

    let _ = test_app
        .post_login(&json!({
            "email":  correct_email,
            "password": correct_password,
        }))
        .await;

    let _ = test_app.post_logout().await;
    let response = test_app.post_logout().await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_given_missing_jwt_when_post_logout_then_400() {
    let test_app = TestApp::build().await.expect("failed to start test app");
    test_app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = test_app.post_logout().await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_given_invalid_jwt_when_post_logout_then_401() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.post_logout().await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
