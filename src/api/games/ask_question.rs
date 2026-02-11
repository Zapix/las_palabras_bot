use actix_web::{HttpResponse, Responder, error, web};
use sqlx::PgPool;

use super::game_response::GameResponse;
use crate::domain::vocabulary::repository::VocabularyDb;
use crate::domain::word_game::converter::Converter;
use crate::domain::word_game::repository::{GameDb, GameRepositoryError, GameTrait};

fn map_ask_question_error(err: GameRepositoryError) -> actix_web::Error {
    match err {
        GameRepositoryError::GameNotFound => error::ErrorNotFound("Game not found"),
        GameRepositoryError::InvalidGameStateTransition => {
            error::ErrorConflict("Invalid game state transition")
        }
        GameRepositoryError::NoWordsAvailable => {
            error::ErrorBadRequest("Cannot ask question: no words available")
        }
        GameRepositoryError::DatabaseError(_) => {
            error::ErrorInternalServerError("Cannot update game in db")
        }
    }
}

#[tracing::instrument(name = "ask_game_question", skip(db_pool), err)]
#[utoipa::path(
    post,
    path = "/api/v1/games/{id}/questions",
    params(
        ("id" = String, Path, description = "ID of the game to ask next question for as uuid"),
    ),
    responses(
        (status = 200, description = "Question asked", body = GameResponse),
        (status = 400, description = "No words available"),
        (status = 404, description = "Game not found"),
        (status = 409, description = "Invalid game state transition"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Games"
)]
pub async fn ask_question(
    db_pool: web::Data<PgPool>,
    game_id: web::Path<uuid::Uuid>,
) -> Result<impl Responder, actix_web::Error> {
    let mut game_repo = GameDb::new(db_pool.as_ref());
    let game = game_repo
        .ask_question(*game_id)
        .await
        .map_err(map_ask_question_error)?;

    let vocabulary_repo = VocabularyDb::new(db_pool.as_ref());
    let converter = Converter::new(&vocabulary_repo);
    let output = converter
        .convert_game_to_output(&game)
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot convert game output"))?;

    Ok(HttpResponse::Ok().json(GameResponse::from(output)))
}
