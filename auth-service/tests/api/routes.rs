use crate::helpers::TestApp;
use reqwest::StatusCode;

#[tokio::test]
async fn test_health_endpoint_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.get_heatlh().await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text().await.unwrap(), "OK");
}

#[tokio::test]
async fn test_root_endpoint_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.get_root().await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("Content-Type").unwrap(), "text/html");
}

#[tokio::test]
async fn test_signup_endpoint_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.post_signup().await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_login_endpoint_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.post_login().await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_verify_2fa_endpoint_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.post_verify_2fa().await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_logout_endpoint_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.post_logout().await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_verify_token_endpoint_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.post_verify_token().await;

    assert_eq!(response.status(), StatusCode::OK);
}
