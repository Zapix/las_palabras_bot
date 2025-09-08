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
    let parsed: serde_json::Value = serde_json::from_str(&body).expect("Failed to parse JSON response");
    assert!(
        parsed.get("version").is_some(),
        "Expected 'version' field in JSON response"
    );
    let _ = app.drop_database().await;
}
