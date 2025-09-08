use actix_web::{web, HttpResponse, Responder, Error as ActixError };
use sqlx::PgPool;
use crate::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use crate::domain::vocabulary::word::Word;
use crate::domain::vocabulary::raw_word::RawWord;
use super::detail_word_error::DetailWordError;

#[derive(serde::Deserialize, Debug, utoipa::ToSchema)]
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
#[utoipa::path(
    patch,
    path = "/api/v1/vocabulary/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "UUID of the word to update"),
    ),
    request_body = UpdateVocabularyData,
    responses(
        (status = 200, description = "Word updated successfully", body = Word),
        (status = 404, description = "Word not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Vocabulary"
)]
pub async fn update_word(
    db_pool: web::Data<PgPool>,
    path: web::Path<uuid::Uuid>,
    data: web::Json<UpdateVocabularyData>,
) -> Result<impl Responder, ActixError> {
    let vocabulary_repo = VocabularyDb::new(db_pool.as_ref());
    let word = vocabulary_repo
        .get_word_by_id(*path)
        .await
        .map_err(DetailWordError::from)?
        .ok_or_else(DetailWordError::not_found)?;
    let raw_word = data.to_raw_word(&word);

    let word = vocabulary_repo
        .update_word(*path, raw_word)
        .await
        .map_err(DetailWordError::from)?;

    Ok(HttpResponse::Ok().json(word))
}
