use actix_web::{HttpResponse, Responder, error, web};
use serde::Deserialize;
use sqlx::PgPool;

use super::game_response::GameResponse;
use crate::domain::vocabulary::repository::VocabularyDb;
use crate::domain::word_game::GameType;
use crate::domain::word_game::converter::Converter;
use crate::domain::word_game::repository::{GameDb, GameTrait};

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub enum CreateGameType {
    RussianToSpanish,
    SpanishToRussian,
}

impl From<CreateGameType> for GameType {
    fn from(value: CreateGameType) -> Self {
        match value {
            CreateGameType::RussianToSpanish => GameType::RussianToSpanish,
            CreateGameType::SpanishToRussian => GameType::SpanishToRussian,
        }
    }
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameRequest {
    pub game_type: CreateGameType,
}

#[tracing::instrument(name = "create_game", skip(db_pool), err)]
#[utoipa::path(
    post,
    path = "/api/v1/games",
    request_body = CreateGameRequest,
    responses(
        (status = 201, description = "Game created successfully", body = GameResponse),
        (status = 400, description = "Invalid game type"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Games"
)]
pub async fn create_game(
    db_pool: web::Data<PgPool>,
    req: web::Json<CreateGameRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let game_type: GameType = req.into_inner().game_type.into();

    let mut game_repo = GameDb::new(db_pool.as_ref());
    let game = game_repo
        .new_game(game_type)
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot create game in db"))?;

    let vocabulary_repo = VocabularyDb::new(db_pool.as_ref());
    let converter = Converter::new(&vocabulary_repo);
    let output = converter
        .convert_game_to_output(&game)
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot convert game output"))?;

    Ok(HttpResponse::Created().json(GameResponse::from(output)))
}
