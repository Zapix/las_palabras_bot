use anyhow::{Error, Result};
use serde_json::json;
use sqlx::types::Json;

use super::super::light_verb::LightVerb;
use super::super::raw_verb::RawVerb;
use super::super::verb::{
    Imperativo, Indicativo, Perfecto, PerfectoSubjuntivo, Progresivo, Subjuntivo, Verb,
};
use super::traits::VerbsRepository;

pub struct VerbsDb<'a> {
    pool: &'a sqlx::PgPool,
}

impl<'a> VerbsDb<'a> {
    pub fn new(pool: &'a sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl<'a> VerbsRepository for VerbsDb<'a> {
    #[tracing::instrument(skip(self))]
    async fn add_verb(&self, verb: RawVerb) -> Result<Verb> {
        sqlx::query_as!(
            Verb,
            r#"
            INSERT INTO "verb" (verb, perfecto, imperativo, indicativo, progresivo, subjuntivo, perfecto_subjuntivo, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
            RETURNING id,
                      verb,
                      perfecto as "perfecto: Json<Perfecto>", 
                      imperativo as "imperativo: Json<Imperativo>", 
                      indicativo as "indicativo: Json<Indicativo>", 
                      progresivo as "progresivo: Json<Progresivo>", 
                      subjuntivo as "subjuntivo: Json<Subjuntivo>", 
                      perfecto_subjuntivo as "perfecto_subjuntivo: Json<PerfectoSubjuntivo>", 
                      created_at,
                      updated_at
            "#,
            verb.verb,
            json!(verb.perfecto),
            json!(verb.imperativo),
            json!(verb.indicativo),
            json!(verb.progresivo),
            json!(verb.subjuntivo),
            json!(verb.perfecto_subjuntivo),
        )
            .fetch_one(self.pool)
            .await
            .map_err(Error::from)
    }

    #[tracing::instrument(skip(self))]
    async fn add_batch_verbs(&self, verbs: Vec<RawVerb>) -> Result<Vec<Verb>> {
        let mut query_builder = sqlx::QueryBuilder::new(
            r#"
            INSERT INTO "verb" (verb, perfecto, imperativo, indicativo, progresivo, subjuntivo, perfecto_subjuntivo, created_at, updated_at)
            "#,
        );

        query_builder.push_values(verbs.iter(), |mut b, verb| {
            b.push_bind(&verb.verb)
                .push_bind(json!(verb.perfecto))
                .push_bind(json!(verb.imperativo))
                .push_bind(json!(verb.indicativo))
                .push_bind(json!(verb.progresivo))
                .push_bind(json!(verb.subjuntivo))
                .push_bind(json!(verb.perfecto_subjuntivo))
                .push_bind(sqlx::types::chrono::Utc::now()) // created_at
                .push_bind(sqlx::types::chrono::Utc::now()); // updated_at
        });

        query_builder.push(
            r#"
            RETURNING id,
                  verb,
                  perfecto, 
                  imperativo, 
                  indicativo, 
                  progresivo, 
                  subjuntivo, 
                  perfecto_subjuntivo, 
                  created_at,
                  updated_at
            "#,
        );

        let query = query_builder.build_query_as::<Verb>();
        let verbs = query.fetch_all(self.pool).await.map_err(Error::from)?;

        Ok(verbs)
    }

    #[tracing::instrument(skip(self))]
    async fn list_verbs(&self, page: u64, per_page: u64) -> Result<Vec<LightVerb>> {
        sqlx::query_as!(
            LightVerb,
            r#"
            SELECT id, verb, created_at, updated_at
            FROM "verb"
            ORDER BY verb
            LIMIT $1
            OFFSET $2
            "#,
            per_page as i64,
            (page * per_page) as i64
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::from)
    }

    #[tracing::instrument(skip(self))]
    async fn count_words(&self) -> Result<i64> {
        sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM "verb""#)
            .fetch_one(self.pool)
            .await
            .map_err(Error::from)
    }

    #[tracing::instrument(skip(self))]
    async fn get_verb_by_id(&self, id: uuid::Uuid) -> Result<Option<Verb>> {
        sqlx::query_as!(
            Verb,
            r#"
            SELECT id,
               verb,
               perfecto as "perfecto: Json<Perfecto>", 
               imperativo as "imperativo: Json<Imperativo>", 
               indicativo as "indicativo: Json<Indicativo>", 
               progresivo as "progresivo: Json<Progresivo>", 
               subjuntivo as "subjuntivo: Json<Subjuntivo>", 
               perfecto_subjuntivo as "perfecto_subjuntivo: Json<PerfectoSubjuntivo>", 
               created_at,
               updated_at
            FROM "verb"
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::from)
    }
}
