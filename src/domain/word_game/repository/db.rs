use anyhow::{Error, Result};
use rand::Rng;
use serde_json::json;
use sqlx::types::Json;

use super::super::game::{Game, GameType};
use super::super::game_status::GameStatus;
use super::GameRepositoryError;
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

    async fn load_game_by_id(&mut self, id: uuid::Uuid) -> Result<Option<Game>> {
        sqlx::query_as!(
            Game,
            r#"
            SELECT id,
                   game_type,
                   game_status as "game_status: Json<GameStatus>",
                   questions_asked,
                   correct_answers,
                   created_at,
                   updated_at
            FROM "game"
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::from)
    }

    async fn ask_question(&mut self, game_id: uuid::Uuid) -> Result<Game, GameRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(GameRepositoryError::DatabaseError)?;

        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT id,
                   game_type,
                   game_status as "game_status: Json<GameStatus>",
                   questions_asked,
                   correct_answers,
                   created_at,
                   updated_at
            FROM "game"
            WHERE id = $1
            "#,
            game_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(GameRepositoryError::DatabaseError)?
        .ok_or(GameRepositoryError::GameNotFound)?;

        let game_status = game.game_status.as_ref().map(|s| s.0.clone());
        let can_ask = matches!(
            game_status,
            Some(GameStatus::Initialized) | Some(GameStatus::Answered { .. })
        );
        if !can_ask {
            return Err(GameRepositoryError::InvalidGameStateTransition);
        }

        let word_ids = sqlx::query_scalar!(
            r#"
                SELECT id
                FROM "vocabulary"
                ORDER BY RANDOM()
                LIMIT 4
            "#
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(GameRepositoryError::DatabaseError)?;

        if word_ids.is_empty() {
            return Err(GameRepositoryError::NoWordsAvailable);
        }

        let random_index = {
            let mut rng = rand::rng();
            rng.random_range(0..word_ids.len())
        };
        let correct_word_id = word_ids[random_index];

        let status = GameStatus::Asked {
            word_ids,
            correct_word_id,
        };

        let game = sqlx::query_as!(
            Game,
            r#"
            UPDATE "game"
            SET game_status = $1,
                questions_asked = questions_asked + 1,
                updated_at = NOW()
            WHERE id = $2
            RETURNING id,
                      game_type,
                      game_status as "game_status: Json<GameStatus>",
                      questions_asked,
                      correct_answers,
                      created_at,
                      updated_at
            "#,
            json!(status),
            game_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(GameRepositoryError::DatabaseError)?;

        tx.commit()
            .await
            .map_err(GameRepositoryError::DatabaseError)?;
        Ok(game)
    }

    async fn answer_question(
        &mut self,
        game_id: uuid::Uuid,
        answer: uuid::Uuid,
    ) -> Result<Game, GameRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(GameRepositoryError::DatabaseError)?;

        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT id,
                   game_type,
                   game_status as "game_status: Json<GameStatus>",
                   questions_asked,
                   correct_answers,
                   created_at,
                   updated_at
            FROM "game"
            WHERE id = $1
            "#,
            game_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(GameRepositoryError::DatabaseError)?
        .ok_or(GameRepositoryError::GameNotFound)?;

        let Some(GameStatus::Asked {
            word_ids,
            correct_word_id,
        }) = game.game_status.as_ref().map(|status| status.0.clone())
        else {
            return Err(GameRepositoryError::InvalidGameStateTransition);
        };

        let is_correct = answer == correct_word_id;
        let status = GameStatus::Answered {
            word_ids,
            correct_word_id,
            is_correct,
        };
        let correct_answers_increment = i32::from(is_correct);

        sqlx::query(
            r#"
            UPDATE "game"
            SET game_status = $1,
                correct_answers = correct_answers + $2,
                updated_at = NOW()
            WHERE id = $3
            "#,
        )
        .bind(json!(status))
        .bind(correct_answers_increment)
        .bind(game_id)
        .execute(&mut *tx)
        .await
        .map_err(GameRepositoryError::DatabaseError)?;

        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT id,
                   game_type,
                   game_status as "game_status: Json<GameStatus>",
                   questions_asked,
                   correct_answers,
                   created_at,
                   updated_at
            FROM "game"
            WHERE id = $1
            "#,
            game_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(GameRepositoryError::DatabaseError)?;

        tx.commit()
            .await
            .map_err(GameRepositoryError::DatabaseError)?;

        Ok(game)
    }

    async fn end_game(&mut self, game_id: uuid::Uuid) -> Result<Game, GameRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(GameRepositoryError::DatabaseError)?;

        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT id,
                   game_type,
                   game_status as "game_status: Json<GameStatus>",
                   questions_asked,
                   correct_answers,
                   created_at,
                   updated_at
            FROM "game"
            WHERE id = $1
            "#,
            game_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(GameRepositoryError::DatabaseError)?
        .ok_or(GameRepositoryError::GameNotFound)?;

        let can_end = matches!(
            game.game_status.as_ref().map(|status| status.0.clone()),
            Some(GameStatus::Initialized) | Some(GameStatus::Answered { .. })
        );
        if !can_end {
            return Err(GameRepositoryError::InvalidGameStateTransition);
        }

        sqlx::query(
            r#"
            UPDATE "game"
            SET game_status = $1,
                updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(json!(GameStatus::Ended))
        .bind(game_id)
        .execute(&mut *tx)
        .await
        .map_err(GameRepositoryError::DatabaseError)?;

        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT id,
                   game_type,
                   game_status as "game_status: Json<GameStatus>",
                   questions_asked,
                   correct_answers,
                   created_at,
                   updated_at
            FROM "game"
            WHERE id = $1
            "#,
            game_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(GameRepositoryError::DatabaseError)?;

        tx.commit()
            .await
            .map_err(GameRepositoryError::DatabaseError)?;

        Ok(game)
    }
}
