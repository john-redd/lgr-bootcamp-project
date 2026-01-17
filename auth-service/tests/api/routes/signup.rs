use crate::helpers::TestApp;
use reqwest::StatusCode;
use serde_json::{Value, json};

#[tokio::test]
async fn test_given_valid_request_body_when_post_signup_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app
        .post_signup(Some(
            json!({
                "email": "john.doe@example.com",
                "password": "password123",
                "requires2FA": false
            })
            .to_string(),
        ))
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        response.json::<Value>().await.unwrap(),
        json!({ "message": "User created successfully!"})
    );
}

#[tokio::test]
async fn test_given_empty_request_body_when_post_signup_then_400() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.post_signup(None).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_given_incorrect_request_body_when_post_signup_then_422() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let test_cases = vec![
        json!({
            "email": "john.doe@example.com",
            "password": "password123",
            "requires2FA": "true",
        })
        .to_string(),
        json!({
            "email": "john.doe@example.com"
        })
        .to_string(),
        json!({
            "email": "john.doe@example.com",
            "password": "password123",
        })
        .to_string(),
        json!({
            "email": "john.doe@example.com",
            "pwd": "password123",
        })
        .to_string(),
    ];

    for test_case in test_cases {
        let response = test_app.post_signup(Some(test_case)).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
