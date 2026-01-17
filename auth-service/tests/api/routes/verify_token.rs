use crate::helpers::TestApp;
use reqwest::StatusCode;

#[tokio::test]
async fn test_verify_token_endpoint_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.post_verify_token().await;

    assert_eq!(response.status(), StatusCode::OK);
}
