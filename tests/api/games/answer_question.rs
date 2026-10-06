use crate::helpers::{TestApp, spawn_app};
use las_palabras_bot::domain::vocabulary::part_of_speech::PartOfSpeech;
use las_palabras_bot::domain::vocabulary::raw_word::RawWord;
use las_palabras_bot::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use las_palabras_bot::domain::word_game::GameStatus;
use las_palabras_bot::domain::word_game::repository::{GameDb, GameTrait};
use uuid::Uuid;

async fn seed_words(app: &mut TestApp) {
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

async fn create_game(app: &TestApp) -> String {
    let response = app
        .api_client()
        .post(format!("{}/api/v1/games", app.address()))
        .json(&serde_json::json!({ "gameType": "RussianToSpanish" }))
        .send()
        .await
        .expect("Failed to create game");
    assert_eq!(response.status().as_u16(), 201);

    response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to decode create response")
        .get("id")
        .and_then(|v| v.as_str())
        .expect("id should be present")
        .to_string()
}

async fn ask_question(app: &TestApp, game_id: &str) {
    let response = app
        .api_client()
        .post(format!(
            "{}/api/v1/games/{}/questions",
            app.address(),
            game_id
        ))
        .send()
        .await
        .expect("Failed to ask question");
    assert_eq!(response.status().as_u16(), 200);
}

async fn answer_question(app: &TestApp, game_id: &str, answer: Uuid) -> reqwest::Response {
    app.api_client()
        .post(format!(
            "{}/api/v1/games/{}/answers",
            app.address(),
            game_id
        ))
        .json(&serde_json::json!({ "answer": answer }))
        .send()
        .await
        .expect("Failed to send answer request")
}

/// Returns (correct_word_id, some_incorrect_word_id) of the currently asked question.
async fn load_asked_word_ids(app: &mut TestApp, game_id: &str) -> (Uuid, Uuid) {
    let game_id = Uuid::parse_str(game_id).expect("valid game id");
    let mut game_repo = GameDb::new(app.db_pool());
    let game = game_repo
        .load_game_by_id(game_id)
        .await
        .expect("Failed to load game")
        .expect("Game should exist");

    match game.game_status.map(|s| s.0) {
        Some(GameStatus::Asked {
            word_ids,
            correct_word_id,
        }) => {
            let incorrect = word_ids
                .into_iter()
                .find(|id| *id != correct_word_id)
                .expect("an incorrect variant should exist");
            (correct_word_id, incorrect)
        }
        other => panic!("game should be in Asked state, got {other:?}"),
    }
}

#[tokio::test]
async fn test_answer_question_404_when_game_not_found() {
    let app = spawn_app().await.expect("Failed to spawn app");

    let response = answer_question(&app, &Uuid::new_v4().to_string(), Uuid::new_v4()).await;

    assert_eq!(response.status().as_u16(), 404);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_answer_question_400_for_invalid_body() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let game_id = create_game(&app).await;

    let response = app
        .api_client()
        .post(format!(
            "{}/api/v1/games/{}/answers",
            app.address(),
            game_id
        ))
        .json(&serde_json::json!({ "answer": "not-a-uuid" }))
        .send()
        .await
        .expect("Failed to send answer request");

    assert_eq!(response.status().as_u16(), 400);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_answer_question_409_when_question_not_asked() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let game_id = create_game(&app).await;

    let response = answer_question(&app, &game_id, Uuid::new_v4()).await;

    assert_eq!(response.status().as_u16(), 409);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_answer_question_200_for_correct_answer() {
    let mut app = spawn_app().await.expect("Failed to spawn app");
    seed_words(&mut app).await;
    let game_id = create_game(&app).await;
    ask_question(&app, &game_id).await;
    let (correct, _) = load_asked_word_ids(&mut app, &game_id).await;

    let response = answer_question(&app, &game_id, correct).await;

    assert_eq!(response.status().as_u16(), 200);
    let body = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to decode response");
    assert_eq!(
        body.get("id").and_then(|v| v.as_str()),
        Some(game_id.as_str())
    );
    assert_eq!(
        body.get("status").and_then(|v| v.as_str()),
        Some("Answered")
    );
    assert_eq!(
        body.get("state")
            .and_then(|v| v.get("isCorrect"))
            .and_then(|v| v.as_bool()),
        Some(true)
    );
    assert_eq!(body.get("questionsAsked").and_then(|v| v.as_i64()), Some(1));
    assert_eq!(body.get("correctAnswers").and_then(|v| v.as_i64()), Some(1));

    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_answer_question_200_for_incorrect_answer() {
    let mut app = spawn_app().await.expect("Failed to spawn app");
    seed_words(&mut app).await;
    let game_id = create_game(&app).await;
    ask_question(&app, &game_id).await;
    let (_, incorrect) = load_asked_word_ids(&mut app, &game_id).await;

    let response = answer_question(&app, &game_id, incorrect).await;

    assert_eq!(response.status().as_u16(), 200);
    let body = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to decode response");
    assert_eq!(
        body.get("status").and_then(|v| v.as_str()),
        Some("Answered")
    );
    assert_eq!(
        body.get("state")
            .and_then(|v| v.get("isCorrect"))
            .and_then(|v| v.as_bool()),
        Some(false)
    );
    assert_eq!(body.get("questionsAsked").and_then(|v| v.as_i64()), Some(1));
    assert_eq!(body.get("correctAnswers").and_then(|v| v.as_i64()), Some(0));

    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_answer_question_409_when_already_answered() {
    let mut app = spawn_app().await.expect("Failed to spawn app");
    seed_words(&mut app).await;
    let game_id = create_game(&app).await;
    ask_question(&app, &game_id).await;
    let (correct, _) = load_asked_word_ids(&mut app, &game_id).await;

    let first_response = answer_question(&app, &game_id, correct).await;
    assert_eq!(first_response.status().as_u16(), 200);

    let second_response = answer_question(&app, &game_id, correct).await;

    assert_eq!(second_response.status().as_u16(), 409);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_concurrent_answer_question_only_accepts_one_answer() {
    let mut app = spawn_app().await.expect("Failed to spawn app");
    seed_words(&mut app).await;
    let game_id = create_game(&app).await;
    ask_question(&app, &game_id).await;
    let (correct, _) = load_asked_word_ids(&mut app, &game_id).await;

    let (first_response, second_response) = tokio::join!(
        answer_question(&app, &game_id, correct),
        answer_question(&app, &game_id, correct)
    );

    let mut statuses = [
        first_response.status().as_u16(),
        second_response.status().as_u16(),
    ];
    statuses.sort_unstable();
    assert_eq!(statuses, [200, 409]);
    let _ = app.drop_database().await;
}
