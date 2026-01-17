use crate::helpers::TestApp;
use reqwest::StatusCode;

#[tokio::test]
async fn test_verify_2fa_endpoint_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.post_verify_2fa().await;

    assert_eq!(response.status(), StatusCode::OK);
}
