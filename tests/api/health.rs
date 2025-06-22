use crate::helpers::spawn_app;

#[tokio::test]
async fn test_health() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let response = app
        .api_client()
        .get(format!("{}/health", app.address()))
        .send()
        .await
        .expect("Failed to send request");
    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );
    assert_eq!(
        "OK",
        response.text().await.expect("Failed to read response text"),
        "Expected 'OK' in response body"
    );
}
