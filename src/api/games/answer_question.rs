use actix_web::{HttpResponse, Responder, error, web};
use serde::Deserialize;
use sqlx::PgPool;

use super::game_response::GameResponse;
use crate::domain::vocabulary::repository::VocabularyDb;
use crate::domain::word_game::converter::Converter;
use crate::domain::word_game::repository::{GameDb, GameRepositoryError, GameTrait};

#[derive(Debug, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnswerQuestionRequest {
    /// ID of the chosen variant as uuid
    pub answer: uuid::Uuid,
}

fn map_answer_question_error(err: GameRepositoryError) -> actix_web::Error {
    match err {
        GameRepositoryError::GameNotFound => error::ErrorNotFound("Game not found"),
        GameRepositoryError::InvalidGameStateTransition => {
            error::ErrorConflict("Invalid game state transition")
        }
        GameRepositoryError::NoWordsAvailable | GameRepositoryError::DatabaseError(_) => {
            error::ErrorInternalServerError("Cannot update game in db")
        }
    }
}

#[tracing::instrument(name = "answer_game_question", skip(db_pool), err)]
#[utoipa::path(
    post,
    path = "/api/v1/games/{id}/answers",
    params(
        ("id" = String, Path, description = "ID of the game to answer current question for as uuid"),
    ),
    request_body = AnswerQuestionRequest,
    responses(
        (status = 200, description = "Question answered", body = GameResponse),
        (status = 400, description = "Invalid request body"),
        (status = 404, description = "Game not found"),
        (status = 409, description = "Invalid game state transition"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Games"
)]
pub async fn answer_question(
    db_pool: web::Data<PgPool>,
    game_id: web::Path<uuid::Uuid>,
    req: web::Json<AnswerQuestionRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let mut game_repo = GameDb::new(db_pool.as_ref());
    let game = game_repo
        .answer_question(*game_id, req.answer)
        .await
        .map_err(map_answer_question_error)?;

    let vocabulary_repo = VocabularyDb::new(db_pool.as_ref());
    let converter = Converter::new(&vocabulary_repo);
    let output = converter
        .convert_game_to_output(&game)
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot convert game output"))?;

    Ok(HttpResponse::Ok().json(GameResponse::from(output)))
}
