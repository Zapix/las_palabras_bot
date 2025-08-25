use anyhow::{Error, Result};

use crate::domain::vocabulary::raw_word::RawWord;
use crate::domain::vocabulary::word::Word;

use super::VocabularyTrait;

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

    async fn get_word_by_id(&self, _id: uuid::Uuid) -> Result<Option<Word>> {
        todo!("Implement create batch words");
    }
    async fn verify_word(&self, _id: uuid::Uuid) -> Result<Word> {
        todo!("Implement create batch words");
    }
    async fn update_word(&self, _id: uuid::Uuid, _raw_word: RawWord) -> Result<Word> {
        todo!("Implement create batch words");
    }
    async fn delete_word(&self, _id: uuid::Uuid) -> Result<()> {
        todo!("Implement create batch words");
    }
}
