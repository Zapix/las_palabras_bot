use actix_web::{web, HttpResponse, Responder, Error as ActixError, ResponseError};
use thiserror::Error;
use sqlx::PgPool;
use crate::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use crate::domain::vocabulary::word::Word;
use crate::domain::vocabulary::raw_word::RawWord;

#[derive(Debug, Error)]
enum UpdateWordError{
    #[error("Word not found")]
    NotFound,
    #[error("Database error: {0}")]
    InternalServerError(String),
}

impl UpdateWordError {
    fn not_found() -> Self {
        UpdateWordError::NotFound
    }
}

impl From<anyhow::Error> for UpdateWordError {
    fn from(err: anyhow::Error) -> Self {
        UpdateWordError::InternalServerError(err.to_string())
    }
}

impl ResponseError for UpdateWordError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateWordError::NotFound => HttpResponse::NotFound().body(self.to_string()),
            UpdateWordError::InternalServerError(_) => {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateVocabularyData {
    spanish: Option<String>,
    russian: Option<String>,
    part_of_speech: Option<String>,
    is_verified: Option<bool>,
}

impl UpdateVocabularyData {
    fn to_raw_word(&self, existing: &Word) -> RawWord {
        RawWord {
            spanish: self
                .spanish
                .clone()
                .unwrap_or_else(|| existing.spanish.clone()),
            russian: self
                .russian
                .clone()
                .unwrap_or_else(|| existing.russian.clone()),

            part_of_speech: self
                .part_of_speech
                .clone()
                .unwrap_or_else(|| existing.part_of_speech.as_str().to_string())
                .into(),
            is_verified: Some(self.is_verified.unwrap_or(existing.is_verified)),
        }
    }
}

#[tracing::instrument(name = "update_word" skip(db_pool))]
pub async fn update_word(
    db_pool: web::Data<PgPool>,
    path: web::Path<uuid::Uuid>,
    data: web::Json<UpdateVocabularyData>,
) -> Result<impl Responder, ActixError> {
    let vocabulary_repo = VocabularyDb::new(db_pool.as_ref());
    let word = vocabulary_repo
        .get_word_by_id(*path)
        .await
        .map_err(UpdateWordError::from)?
        .ok_or_else(UpdateWordError::not_found)?;
    let raw_word = data.to_raw_word(&word);

    let word = vocabulary_repo
        .update_word(*path, raw_word)
        .await
        .map_err(UpdateWordError::from)?;

    Ok(HttpResponse::Ok().json(word))
}
