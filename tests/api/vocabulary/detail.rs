use crate::helpers::spawn_app;
use las_palabras_bot::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};

#[tokio::test]
async fn test_detail_404_error() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let response = app
        .api_client()
        .get(format!(
            "{}/api/v1/vocabulary/9ed64d96-8342-478f-aa1f-c23f9c61d9c7",
            app.address()
        ))
        .send()
        .await
        .expect("Failed to send request");
    assert_eq!(response.status().as_u16(), 404);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_detail_word() {
    let mut app = spawn_app().await.expect("Failed to spawn app");
    let db_pool = app.db_pool();
    println!("Database pool info: {:?}", db_pool.connect_options());
    let vocabulary_db = VocabularyDb::new(db_pool);
    let created_word = vocabulary_db
        .create_word(las_palabras_bot::domain::vocabulary::raw_word::RawWord {
            spanish: "perro".into(),
            part_of_speech: "noun".into(),
            russian: "собака".into(),
            is_verified: Some(false),
        })
        .await
        .expect("Failed to create word");

    let response = app
        .api_client()
        .get(format!(
            "{}/api/v1/vocabulary/{}",
            app.address(),
            created_word.id
        ))
        .send()
        .await
        .expect("Failed to send request");
    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );

    let word: las_palabras_bot::domain::vocabulary::word::Word = response
        .json()
        .await
        .expect("Failed to parse JSON response");

    assert_eq!(word.spanish, "perro");
    assert_eq!(word.russian, "собака");
    assert_eq!(word.part_of_speech, "noun".into());
    let _ = app.drop_database().await;
}
