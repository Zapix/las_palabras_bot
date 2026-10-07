use actix_web::{HttpResponse, Responder, error, web};
use serde::Serialize;
use sqlx::PgPool;

use crate::domain::word_game::Game;
use crate::domain::word_game::repository::{GameDb, GameRepositoryError, GameTrait};

#[derive(Debug, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndGameResponse {
    pub status: String,
    pub questions_asked: i32,
    pub correct_answers: i32,
}

impl From<Game> for EndGameResponse {
    fn from(value: Game) -> Self {
        Self {
            status: "Ended".to_string(),
            questions_asked: value.questions_asked,
            correct_answers: value.correct_answers,
        }
    }
}

fn map_end_game_error(err: GameRepositoryError) -> actix_web::Error {
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

#[tracing::instrument(name = "end_game", skip(db_pool), err)]
#[utoipa::path(
    post,
    path = "/api/v1/games/{id}/end",
    params(
        ("id" = String, Path, description = "ID of the game to end as uuid"),
    ),
    responses(
        (status = 200, description = "Game ended", body = EndGameResponse),
        (status = 404, description = "Game not found"),
        (status = 409, description = "Invalid game state transition"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Games"
)]
pub async fn end_game(
    db_pool: web::Data<PgPool>,
    game_id: web::Path<uuid::Uuid>,
) -> Result<impl Responder, actix_web::Error> {
    let mut game_repo = GameDb::new(db_pool.as_ref());
    let game = game_repo
        .end_game(*game_id)
        .await
        .map_err(map_end_game_error)?;

    Ok(HttpResponse::Ok().json(EndGameResponse::from(game)))
}
