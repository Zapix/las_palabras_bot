use crate::helpers::spawn_app;
use las_palabras_bot::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use las_palabras_bot::domain::vocabulary::word::Word;
use las_palabras_bot::domain::vocabulary::raw_word::RawWord;

#[tokio::test]
async fn update_word_returns_404_for_non_existing_word() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();
    let new_word = serde_json::json!({
        "spanish": "updated_word",
        "part_of_speech": "verb",
        "russian": "обновленное_слово"
    });
    let response = client
        .put(format!("{}/api/v1/vocabulary/9ed64d96-8342-478f-aa1f-c23f9c61d9c7", &app.address()))
        .header("Content-Type", "application/json")
        .json(&new_word)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status().as_u16(), 404);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn update_word_succeeds_for_existing_word() {
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
    let new_word = serde_json::json!({
        "spanish": "updated_word",
        "part_of_speech": "verb",
        "russian": "обновленное_слово"
    });
    let response = client
        .put(format!("{}/api/v1/vocabulary/{}", &app.address(), created_word.id))
        .header("Content-Type", "application/json")
        .json(&new_word)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );
    let updated_word: Word = response
        .json()
        .await
        .expect("Failed to parse response JSON.");

    assert_eq!(updated_word.id, created_word.id);
    assert_eq!(updated_word.spanish, "updated_word");
    assert_eq!(updated_word.part_of_speech, "verb".into());
    assert_eq!(updated_word.russian, "обновленное_слово");
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn update_word_verified_through_patch() {
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

    assert!(!created_word.is_verified);
    let patch = serde_json::json!({
        "is_verified": true
    });

    let client = reqwest::Client::new();
    let response = client
        .patch(format!(
            "{}/api/v1/vocabulary/{}",
            &app.address(),
            created_word.id
        ))
        .header("Content-Type", "application/json")
        .json(&patch)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );
    let updated_word: Word = response
        .json()
        .await
        .expect("Failed to parse response JSON.");

    assert_eq!(updated_word.id, created_word.id);
    assert!(updated_word.is_verified);
    let _ = app.drop_database().await;
}