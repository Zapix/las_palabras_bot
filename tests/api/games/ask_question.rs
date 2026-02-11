use crate::helpers::spawn_app;
use las_palabras_bot::domain::vocabulary::part_of_speech::PartOfSpeech;
use las_palabras_bot::domain::vocabulary::raw_word::RawWord;
use las_palabras_bot::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use uuid::Uuid;

async fn seed_words(app: &mut crate::helpers::TestApp) {
    let vocabulary = VocabularyDb::new(app.db_pool());
    let words = [
        ("perro", "собака"),
        ("gato", "кошка"),
        ("casa", "дом"),
        ("libro", "книга"),
    ];

    for (spanish, russian) in words {
        vocabulary
            .create_word(RawWord {
                spanish: spanish.to_string(),
                russian: russian.to_string(),
                part_of_speech: PartOfSpeech::Noun,
                is_verified: Some(false),
            })
            .await
            .expect("Failed to create test word");
    }
}

#[tokio::test]
async fn test_ask_question_404_when_game_not_found() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let non_existent_game = Uuid::new_v4();

    let response = app
        .api_client()
        .post(format!(
            "{}/api/v1/games/{}/questions",
            app.address(),
            non_existent_game
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 404);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_ask_question_400_when_no_words_available() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let req = serde_json::json!({
        "gameType": "RussianToSpanish"
    });

    let create_response = app
        .api_client()
        .post(format!("{}/api/v1/games", app.address()))
        .json(&req)
        .send()
        .await
        .expect("Failed to create game");
    assert_eq!(create_response.status().as_u16(), 201);

    let created = create_response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to decode create response");
    let game_id = created
        .get("id")
        .and_then(|v| v.as_str())
        .expect("id should be present");

    let response = app
        .api_client()
        .post(format!(
            "{}/api/v1/games/{}/questions",
            app.address(),
            game_id
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 400);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_ask_question_200() {
    let mut app = spawn_app().await.expect("Failed to spawn app");
    seed_words(&mut app).await;

    let req = serde_json::json!({
        "gameType": "RussianToSpanish"
    });

    let create_response = app
        .api_client()
        .post(format!("{}/api/v1/games", app.address()))
        .json(&req)
        .send()
        .await
        .expect("Failed to create game");
    assert_eq!(create_response.status().as_u16(), 201);

    let created = create_response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to decode create response");
    let game_id = created
        .get("id")
        .and_then(|v| v.as_str())
        .expect("id should be present");

    let response = app
        .api_client()
        .post(format!(
            "{}/api/v1/games/{}/questions",
            app.address(),
            game_id
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 200);
    let body = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to decode response");

    assert_eq!(body.get("id").and_then(|v| v.as_str()), Some(game_id));
    assert_eq!(
        body.get("status").and_then(|v| v.as_str()),
        Some("Asked"),
        "status should be Asked after asking a question"
    );
    assert_eq!(
        body.get("state")
            .and_then(|v| v.get("status"))
            .and_then(|v| v.as_str()),
        Some("Asked")
    );
    assert!(
        body.get("state")
            .and_then(|v| v.get("question"))
            .and_then(|v| v.as_str())
            .is_some()
    );
    assert_eq!(
        body.get("state")
            .and_then(|v| v.get("variants"))
            .and_then(|v| v.as_array())
            .map(Vec::len),
        Some(4)
    );
    assert_eq!(body.get("questionsAsked").and_then(|v| v.as_i64()), Some(1));

    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_ask_question_409_for_invalid_transition() {
    let mut app = spawn_app().await.expect("Failed to spawn app");
    seed_words(&mut app).await;

    let req = serde_json::json!({
        "gameType": "RussianToSpanish"
    });

    let create_response = app
        .api_client()
        .post(format!("{}/api/v1/games", app.address()))
        .json(&req)
        .send()
        .await
        .expect("Failed to create game");
    assert_eq!(create_response.status().as_u16(), 201);

    let created = create_response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to decode create response");
    let game_id = created
        .get("id")
        .and_then(|v| v.as_str())
        .expect("id should be present");

    let first_response = app
        .api_client()
        .post(format!(
            "{}/api/v1/games/{}/questions",
            app.address(),
            game_id
        ))
        .send()
        .await
        .expect("Failed to send first request");
    assert_eq!(first_response.status().as_u16(), 200);

    let second_response = app
        .api_client()
        .post(format!(
            "{}/api/v1/games/{}/questions",
            app.address(),
            game_id
        ))
        .send()
        .await
        .expect("Failed to send second request");

    assert_eq!(second_response.status().as_u16(), 409);
    let _ = app.drop_database().await;
}
