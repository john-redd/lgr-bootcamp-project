use crate::helpers::TestApp;
use reqwest::StatusCode;

#[tokio::test]
async fn test_login_endpoint_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.post_login().await;

    assert_eq!(response.status(), StatusCode::OK);
}
