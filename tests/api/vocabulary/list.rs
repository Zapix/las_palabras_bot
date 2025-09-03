use crate::helpers::spawn_app;
use las_palabras_bot::api::pagination::Pagination;
use las_palabras_bot::domain::vocabulary::raw_word::RawWord;
use las_palabras_bot::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use las_palabras_bot::domain::vocabulary::word::Word;

#[tokio::test]
async fn test_list() {
    let mut app = spawn_app().await.expect("Failed to spawn app");

    let db_pool = app.db_pool();
    println!("Database pool info: {:?}", db_pool.connect_options());
    let vocabulary_db = VocabularyDb::new(db_pool);
    vocabulary_db
        .create_batch_words(vec![
            RawWord {
                spanish: "hola".into(),
                part_of_speech: "noun".into(),
                russian: "привет".into(),
                is_verified: Some(false),
            },
            RawWord {
                spanish: "adiós".into(),
                part_of_speech: "noun".into(),
                russian: "прощай".into(),
                is_verified: Some(false),
            },
        ])
        .await
        .expect("Failed to create word");

    let response = app
        .api_client()
        .get(format!("{}/api/v1/vocabulary", app.address()))
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );

    let data: Pagination<Word> = response
        .json()
        .await
        .expect("Failed to parse JSON response");

    assert!(data.total >= 2, "Expected at least 2 words in the response");
    assert!(
        data.items.iter().any(|w| w.spanish == "hola"),
        "Expected to find the word 'hola'"
    );
}

#[tokio::test]
async fn test_list_with_verified_filter() {
    let mut app = spawn_app().await.expect("Failed to spawn app");

    let db_pool = app.db_pool();
    println!("Database pool info: {:?}", db_pool.connect_options());
    let vocabulary_db = VocabularyDb::new(db_pool);
    vocabulary_db
        .create_batch_words(vec![
            RawWord {
                spanish: "hola".into(),
                part_of_speech: "noun".into(),
                russian: "привет".into(),
                is_verified: Some(true),
            },
            RawWord {
                spanish: "adiós".into(),
                part_of_speech: "noun".into(),
                russian: "прощай".into(),
                is_verified: Some(false),
            },
        ])
        .await
        .expect("Failed to create word");

    let response = app
        .api_client()
        .get(format!(
            "{}/api/v1/vocabulary?is_verified=true",
            app.address()
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );

    let data: Pagination<Word> = response
        .json()
        .await
        .expect("Failed to parse JSON response");

    assert!(data.total >= 1, "Expected at least 1 verified word in the response");
    assert!(
        data.items.iter().all(|w| w.is_verified),
        "Expected all words to be verified"
    );
}

#[tokio::test]
async fn test_list_with_unverified_filter() {
    let mut app = spawn_app().await.expect("Failed to spawn app");

    let db_pool = app.db_pool();
    println!("Database pool info: {:?}", db_pool.connect_options());
    let vocabulary_db = VocabularyDb::new(db_pool);
    vocabulary_db
        .create_batch_words(vec![
            RawWord {
                spanish: "hola".into(),
                part_of_speech: "noun".into(),
                russian: "привет".into(),
                is_verified: Some(true),
            },
            RawWord {
                spanish: "adiós".into(),
                part_of_speech: "noun".into(),
                russian: "прощай".into(),
                is_verified: Some(false),
            },
        ])
        .await
        .expect("Failed to create word");

    let response = app
        .api_client()
        .get(format!(
            "{}/api/v1/vocabulary?is_verified=false",
            app.address()
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );

    let data: Pagination<Word> = response
        .json()
        .await
        .expect("Failed to parse JSON response");

    assert!(
        data.total >= 1,
        "Expected at least 1 unverified word in the response"
    );
    assert!(
        data.items.iter().all(|w| !w.is_verified),
        "Expected all words to be unverified"
    );
}