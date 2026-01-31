use crate::helpers::TestApp;
use fake::Fake;
use fake::faker::internet::raw::SafeEmail;
use fake::locales::EN;
use reqwest::StatusCode;
use serde_json::{Value, json};

#[tokio::test]
async fn test_given_empty_request_body_when_post_login_then_422() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.post_login(&"").await;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_given_incorrect_request_body_when_post_login_then_422() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let test_cases = vec![
        json!({
            "email": "john.doe@example.com",
        }),
        json!({
            "password": "password123!",
        }),
        json!({
            "email": "john.doe@example.com",
            "pwd": "password123!",
        }),
        json!({
            "emal": "john.doe@example.com",
            "password": "password123!",
        }),
    ];

    for test_case in test_cases {
        let response = test_app.post_login(&test_case).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[tokio::test]
async fn test_given_invalid_email_when_post_login_then_400() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let test_cases = vec![
        json!({
            "email": "john.doeexample.com",
            "password": "password123!",
        }),
        json!({
            "email": "",
            "password": "password123!",
        }),
    ];

    for test_case in test_cases {
        let response = test_app.post_login(&test_case).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

#[tokio::test]
async fn test_given_invalid_password_when_post_login_then_400() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let test_cases = vec![json!({
        "email": "john.doeexample.com",
        "password": "1",
    })];

    for test_case in test_cases {
        let response = test_app.post_login(&test_case).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

#[tokio::test]
async fn test_given_invalid_credentials_when_post_login_then_401() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let correct_email: String = SafeEmail(EN).fake();
    let correct_password = "Password123!";
    let valid_user = json!({
        "email":  correct_email,
        "password": correct_password,
    });

    let _ = test_app.post_signup(&valid_user).await;

    let incorrect_email: String = SafeEmail(EN).fake();
    let incorrect_password = "pAssword1234!";

    let test_cases = vec![
        json!({
            "email": incorrect_email,
            "password": correct_password,
        }),
        json!({
            "email": correct_email,
            "password": incorrect_password,
        }),
        json!({
            "email": incorrect_email,
            "password": incorrect_password,
        }),
    ];

    for test_case in test_cases {
        let response = test_app.post_login(&test_case).await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response.json::<Value>().await.unwrap()["error"],
            "Invalid credentials."
        );
    }
}
