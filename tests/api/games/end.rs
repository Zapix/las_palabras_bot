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

async fn answer_correctly(app: &mut TestApp, game_id: &str) {
    let id = Uuid::parse_str(game_id).expect("valid game id");
    let game = GameDb::new(app.db_pool())
        .load_game_by_id(id)
        .await
        .expect("Failed to load game")
        .expect("Game should exist");
    let Some(GameStatus::Asked {
        correct_word_id, ..
    }) = game.game_status.map(|s| s.0)
    else {
        panic!("game should be in Asked state");
    };

    let response = app
        .api_client()
        .post(format!(
            "{}/api/v1/games/{}/answers",
            app.address(),
            game_id
        ))
        .json(&serde_json::json!({ "answer": correct_word_id }))
        .send()
        .await
        .expect("Failed to answer question");
    assert_eq!(response.status().as_u16(), 200);
}

async fn end_game(app: &TestApp, game_id: &str) -> reqwest::Response {
    app.api_client()
        .post(format!("{}/api/v1/games/{}/end", app.address(), game_id))
        .send()
        .await
        .expect("Failed to send end request")
}

#[tokio::test]
async fn test_end_game_404_when_game_not_found() {
    let app = spawn_app().await.expect("Failed to spawn app");

    let response = end_game(&app, &Uuid::new_v4().to_string()).await;

    assert_eq!(response.status().as_u16(), 404);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_end_game_200_for_initialized_game() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let game_id = create_game(&app).await;

    let response = end_game(&app, &game_id).await;

    assert_eq!(response.status().as_u16(), 200);
    let body = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to decode response");
    assert_eq!(
        body,
        serde_json::json!({
            "status": "Ended",
            "questionsAsked": 0,
            "correctAnswers": 0
        })
    );

    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_end_game_200_after_answered_question() {
    let mut app = spawn_app().await.expect("Failed to spawn app");
    seed_words(&mut app).await;
    let game_id = create_game(&app).await;
    ask_question(&app, &game_id).await;
    answer_correctly(&mut app, &game_id).await;

    let response = end_game(&app, &game_id).await;

    assert_eq!(response.status().as_u16(), 200);
    let body = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to decode response");
    assert_eq!(
        body,
        serde_json::json!({
            "status": "Ended",
            "questionsAsked": 1,
            "correctAnswers": 1
        })
    );

    let detail = app
        .api_client()
        .get(format!("{}/api/v1/games/{}", app.address(), game_id))
        .send()
        .await
        .expect("Failed to get game")
        .json::<serde_json::Value>()
        .await
        .expect("Failed to decode detail response");
    assert_eq!(detail.get("status").and_then(|v| v.as_str()), Some("Ended"));

    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_end_game_409_when_question_is_pending() {
    let mut app = spawn_app().await.expect("Failed to spawn app");
    seed_words(&mut app).await;
    let game_id = create_game(&app).await;
    ask_question(&app, &game_id).await;

    let response = end_game(&app, &game_id).await;

    assert_eq!(response.status().as_u16(), 409);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_end_game_409_when_already_ended() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let game_id = create_game(&app).await;

    let first_response = end_game(&app, &game_id).await;
    assert_eq!(first_response.status().as_u16(), 200);

    let second_response = end_game(&app, &game_id).await;

    assert_eq!(second_response.status().as_u16(), 409);
    let _ = app.drop_database().await;
}

#[tokio::test]
async fn test_end_game_concurrent_requests_only_one_succeeds() {
    const ROUNDS: usize = 5;
    const CONCURRENT_REQUESTS: usize = 5;
    let app = spawn_app().await.expect("Failed to spawn app");

    for _ in 0..ROUNDS {
        let game_id = create_game(&app).await;

        let url = format!("{}/api/v1/games/{}/end", app.address(), game_id);
        let mut requests = tokio::task::JoinSet::new();
        for _ in 0..CONCURRENT_REQUESTS {
            let client = app.api_client().clone();
            let url = url.clone();
            requests.spawn(async move {
                client
                    .post(url)
                    .send()
                    .await
                    .expect("Failed to send end request")
                    .status()
                    .as_u16()
            });
        }
        let statuses = requests.join_all().await;

        let ok_count = statuses.iter().filter(|s| **s == 200).count();
        let conflict_count = statuses.iter().filter(|s| **s == 409).count();
        assert_eq!(
            ok_count, 1,
            "exactly one concurrent end request should succeed"
        );
        assert_eq!(conflict_count, CONCURRENT_REQUESTS - 1);
    }

    let _ = app.drop_database().await;
}
