use anyhow::{Error, Result};
use serde_json::json;
use sqlx::types::Json;

use super::super::game::{Game, GameType};
use super::super::game_status::GameStatus;
use super::traits::GameTrait;

pub struct GameDb<'a> {
    pool: &'a sqlx::PgPool,
}

impl<'a> GameDb<'a> {
    pub fn new(pool: &'a sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl<'a> GameTrait for GameDb<'a> {
    async fn new_game(&mut self, game_type: GameType) -> Result<Game> {
        sqlx::query_as!(
            Game,
            r#"
            INSERT INTO "game" (game_type, game_status, questions_asked, correct_answers, created_at, updated_at)
            VALUES ($1, $2, 0, 0, NOW(), NOW())
            RETURNING id,
                      game_type,
                      game_status as "game_status: Json<GameStatus>",
                      questions_asked,
                      correct_answers,
                      created_at,
                      updated_at
            "#,
            game_type.as_str(),
            json!(GameStatus::Initialized),
        )
            .fetch_one(self.pool)
            .await
            .map_err(Error::from)
    }

    /*
    async fn load_game_by_id(&mut self, id: uuid::Uuid) -> Result<Option<Game>> {
        todo!("Load game by id");
    }

    async fn ask_question(&mut self, game_id: uuid::Uuid) -> Result<Game> {
        todo!("Ask question implementaiton");
    }

    async fn answer_question(&mut self, game_id: uuid::Uuid, answer: uuid::Uuid) -> Result<Game> {
        todo!("answer question implementaiton");
    }

    async fn end_game(&mut self, game_id: uuid::Uuid) -> Result<Game> {
        todo!("end game implementaiton");
    }
    */
}
