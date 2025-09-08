use crate::helpers::spawn_app;
use std::collections::HashMap;

#[tokio::test]
async fn test_create_400_error() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let mut new_word = HashMap::<String, String>::new();
    new_word.insert("spanish".into(), "gracias".into());
    let response = app
        .api_client()
        .post(format!("{}/api/v1/vocabulary", app.address()))
        .json(&new_word)
        .send()
        .await
        .expect("Failed to send request");
    assert_eq!(response.status().as_u16(), 400);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_create_word() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let new_word = serde_json::json!({
        "spanish": "gracias",
        "part_of_speech": "noun",
        "russian": "спасибо"
    });
    let response = app
        .api_client()
        .post(format!("{}/api/v1/vocabulary", app.address()))
        .json(&new_word)
        .send()
        .await
        .expect("Failed to send request");
    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );
    let body = response.text().await.expect("Failed to read response text");
    assert!(body.contains("gracias") && body.contains("спасибо") && body.contains("noun"));
    let _ = app.drop_database().await;
}
