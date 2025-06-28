use crate::helpers::spawn_app;

#[tokio::test]
async fn test_info() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let response = app
        .api_client()
        .get(format!("{}/info", app.address()))
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );

    let body = response.text().await.expect("Failed to read response text");
    assert!(
        body.contains("version"),
        "Expected 'version' in response body"
    );
}
