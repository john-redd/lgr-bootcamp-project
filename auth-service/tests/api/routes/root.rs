use crate::helpers::TestApp;
use reqwest::StatusCode;

#[tokio::test]
async fn test_root_endpoint_returns_200() {
    let test_app = TestApp::build().await.expect("failed to start test app");

    let response = test_app.get_root().await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("Content-Type").unwrap(), "text/html");
}
