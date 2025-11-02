use crate::helpers::spawn_app;

#[tokio::test]
async fn test_list() {
    let app = spawn_app().await.expect("Failed to spawn app");

    let response = app
        .api_client()
        .get(format!("{}/api/v1/verbs", app.address()))
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );

    let _ = app.drop_database().await;
}
