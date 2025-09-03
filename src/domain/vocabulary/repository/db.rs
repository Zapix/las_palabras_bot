use anyhow::{Error, Result};

use crate::domain::vocabulary::raw_word::RawWord;
use crate::domain::vocabulary::word::Word;

use super::{IsVerifiedFilter, VocabularyTrait};

pub struct VocabularyDb<'a> {
    pool: &'a sqlx::PgPool,
}

impl<'a> VocabularyDb<'a> {
    pub fn new(pool: &'a sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl<'a> VocabularyTrait for VocabularyDb<'a> {
    #[tracing::instrument(skip(self))]
    async fn create_word(&self, raw_word: RawWord) -> Result<Word> {
        sqlx::query_as!(
            Word,
            r#"
            INSERT INTO "vocabulary" (spanish, russian, part_of_speech, is_verified, created_at, updated_at)
            VALUES ($1, $2, $3, FALSE, NOW(), NOW())
            RETURNING id, spanish, russian, part_of_speech, is_verified, created_at, updated_at
            "#,
            raw_word.spanish,
            raw_word.russian,
            raw_word.part_of_speech.as_str(),
        ).fetch_one(self.pool)
        .await
        .map_err(Error::from)
    }

    #[tracing::instrument(skip(self))]
    async fn create_batch_words(&self, raw_words: Vec<RawWord>) -> Result<Vec<Word>> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
            r#"
            INSERT INTO "vocabulary" (spanish, russian, part_of_speech, is_verified, created_at, updated_at)
            "#,
        );

        query_builder.push_values(raw_words.iter(), |mut b, raw_word| {
            b.push_bind(&raw_word.spanish)
                .push_bind(&raw_word.russian)
                .push_bind(raw_word.part_of_speech.as_str())
                .push_bind(false) // is_verified
                .push_bind(sqlx::types::chrono::Utc::now()) // created_at
                .push_bind(sqlx::types::chrono::Utc::now()); // updated_at
        });

        query_builder.push(
            r#"
            RETURNING id, spanish, russian, part_of_speech, is_verified, created_at, updated_at
            "#,
        );

        let query = query_builder.build_query_as::<Word>();
        let words = query.fetch_all(self.pool).await.map_err(Error::from)?;

        Ok(words)
    }

    #[tracing::instrument(skip(self))]
    async fn list_word(&self, page: u64, per_page: u64, filter: IsVerifiedFilter) -> Result<Vec<Word>> {
        let filter = match filter {
            IsVerifiedFilter::Any => vec![true, false],
            IsVerifiedFilter::True => vec![true],
            IsVerifiedFilter::False => vec![false],
        };

        sqlx::query_as!(
            Word,
            r#"
                SELECT id, spanish, russian, part_of_speech, is_verified, created_at, updated_at
                FROM "vocabulary"
                WHERE is_verified = ANY($1)
                ORDER BY created_at DESC
                OFFSET $2 LIMIT $3
            "#,
            &filter,
            (page * per_page) as i64,
            per_page as i64
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::from)
    }

    #[tracing::instrument(skip(self))]
    async fn count_words(&self) -> Result<i64> {
        sqlx::query_scalar!(
            r#"
                SELECT COUNT(*) as "count!"
                FROM "vocabulary"
            "#
        )
        .fetch_one(self.pool)
        .await
        .map_err(Error::from)
    }

    async fn get_word_by_id(&self, id: uuid::Uuid) -> Result<Option<Word>> {
        sqlx::query_as!(
            Word,
            r#"
            SELECT id, spanish, russian, part_of_speech, is_verified, created_at, updated_at
            FROM "vocabulary"
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::from)
    }

    async fn verify_word(&self, _id: uuid::Uuid) -> Result<Word> {
        todo!("Implement create batch words");
    }
    async fn update_word(&self, id: uuid::Uuid, raw_word: RawWord) -> Result<Word> {
        sqlx::query_as!(
            Word,
            r#"
            UPDATE "vocabulary"
            SET spanish = $1, russian = $2, part_of_speech = $3, is_verified = $4, updated_at = NOW()
            WHERE id = $5
            RETURNING id, spanish, russian, part_of_speech, is_verified, created_at, updated_at
            "#,
            raw_word.spanish,
            raw_word.russian,
            raw_word.part_of_speech.as_str(),
            raw_word.is_verified.unwrap_or(false),
            id
        )
            .fetch_one(self.pool)
            .await
            .map_err(Error::from)
    }
    async fn delete_word(&self, id: uuid::Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM "vocabulary"
            WHERE id = $1
            "#,
            id
        )
        .execute(self.pool)
        .await
        .map_err(Error::from)?;

        Ok(())
    }
}
