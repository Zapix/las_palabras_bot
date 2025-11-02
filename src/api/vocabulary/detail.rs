use super::detail_word_error::DetailWordError;
use crate::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use crate::domain::vocabulary::word::Word;
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

#[tracing::instrument(name = "detail_word", skip(db_pool))]
#[utoipa::path(
    get,
    path = "/api/v1/vocabulary/{id}",
    params(
        ("id" = String, Path, description = "UUID of the word to retrieve"),
    ),
    responses(
        (status = 200, description = "Word details", body = Word),
        (status = 404, description = "Word not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Vocabulary"
)]
pub async fn get_word(
    db_pool: web::Data<PgPool>,
    word_id: web::Path<uuid::Uuid>,
) -> Result<impl Responder, actix_web::Error> {
    let vocabulary_repo = VocabularyDb::new(db_pool.as_ref());
    let word = vocabulary_repo
        .get_word_by_id(*word_id)
        .await
        .map_err(DetailWordError::from)?
        .ok_or_else(DetailWordError::not_found)?;

    Ok(HttpResponse::Ok().json(word))
}
