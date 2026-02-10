use crate::helpers::spawn_app;
use uuid::Uuid;

#[tokio::test]
async fn test_create_game_201() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let req = serde_json::json!({
        "gameType": "RussianToSpanish"
    });

    let response = app
        .api_client()
        .post(format!("{}/api/v1/games", app.address()))
        .json(&req)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 201);
    let body = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to decode response");

    let id = body
        .get("id")
        .and_then(|v| v.as_str())
        .expect("id should be present");
    Uuid::parse_str(id).expect("id should be valid UUID");

    assert_eq!(
        body.get("gameType").and_then(|v| v.as_str()),
        Some("RussianToSpanish")
    );
    assert_eq!(
        body.get("status").and_then(|v| v.as_str()),
        Some("Initialized")
    );
    assert_eq!(body.get("questionsAsked").and_then(|v| v.as_i64()), Some(0));
    assert_eq!(body.get("correctAnswers").and_then(|v| v.as_i64()), Some(0));
    assert!(body.get("createdAt").and_then(|v| v.as_str()).is_some());
    assert!(body.get("updatedAt").and_then(|v| v.as_str()).is_some());

    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_create_game_400_invalid_game_type() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let req = serde_json::json!({
        "gameType": "InvalidType"
    });

    let response = app
        .api_client()
        .post(format!("{}/api/v1/games", app.address()))
        .json(&req)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 400);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_create_game_400_missing_game_type() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let req = serde_json::json!({});

    let response = app
        .api_client()
        .post(format!("{}/api/v1/games", app.address()))
        .json(&req)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 400);
    let _ = app.drop_database().await;
}
