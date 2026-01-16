use crate::helpers::run_app;

#[tokio::test]
async fn test_app_starts() {
    let result = run_app().await;

    assert!(result.is_ok());
    if let Ok(address) = result {
        assert!(!address.is_empty())
    };
}
