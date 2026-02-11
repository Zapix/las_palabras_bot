use actix_web::{HttpResponse, Responder, error, web};
use sqlx::PgPool;

use super::game_response::GameResponse;
use crate::domain::vocabulary::repository::VocabularyDb;
use crate::domain::word_game::converter::Converter;
use crate::domain::word_game::repository::{GameDb, GameTrait};

#[tracing::instrument(name = "get_game", skip(db_pool), err)]
#[utoipa::path(
    get,
    path = "/api/v1/games/{id}",
    params(
        ("id" = String, Path, description = "ID of the game to retrieve as uuid"),
    ),
    responses(
        (status = 200, description = "Game found", body = GameResponse),
        (status = 404, description = "Game not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Games"
)]
pub async fn get_game(
    db_pool: web::Data<PgPool>,
    game_id: web::Path<uuid::Uuid>,
) -> Result<impl Responder, actix_web::Error> {
    let mut game_repo = GameDb::new(db_pool.as_ref());
    let game = game_repo
        .load_game_by_id(*game_id)
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot load game from db"))?
        .ok_or_else(|| error::ErrorNotFound("Game not found"))?;

    let vocabulary_repo = VocabularyDb::new(db_pool.as_ref());
    let converter = Converter::new(&vocabulary_repo);
    let output = converter
        .convert_game_to_output(&game)
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot convert game output"))?;

    Ok(HttpResponse::Ok().json(GameResponse::from(output)))
}
