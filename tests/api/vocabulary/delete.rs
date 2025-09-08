use crate::helpers::spawn_app;
use reqwest::StatusCode;
use las_palabras_bot::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use las_palabras_bot::domain::vocabulary::raw_word::RawWord;

#[tokio::test]
async fn delete_word_returns_404_for_non_existing_word() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();
    let response = client
        .delete(format!(
            "{}/api/v1/vocabulary/9ed64d96-8342-478f-aa1f-c23f9c61d9c7",
            &app.address()
        ))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status().as_u16(), 404);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn delete_word_succeeds_for_existing_word() {
    let mut app = spawn_app().await.expect("Failed to spawn app");
    let new_word = RawWord {
        spanish: "test_word".into(),
        part_of_speech: "noun".into(),
        russian: "тестовое_слово".into(),
        is_verified: Some(false),
    };
    let db_pool = app.db_pool();
    let vocabulary_db = VocabularyDb::new(db_pool);
    let created_word = vocabulary_db
        .create_word(new_word)
        .await
        .expect("Failed to create word");
    let client = reqwest::Client::new();
    let response = client
        .delete(format!(
            "{}/api/v1/vocabulary/{}",
            &app.address(),
            created_word.id
        ))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );
    assert_eq!(
        response.status(),
        StatusCode::NO_CONTENT,
        "Expected 204 No Content status"
    );
    let _ = app.drop_database().await;
}