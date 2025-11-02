use super::detail_word_error::DetailWordError;
use crate::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use actix_web::{Error as ActixError, HttpResponse, Responder, web};
use sqlx::PgPool;

#[tracing::instrument(name = "delete_word" skip(db_pool))]
#[utoipa::path(
    delete,
    path = "/api/v1/vocabulary/{id}",
    params(
        ("id" = String, Path, description = "UUID of the word to delete"),
    ),
    responses(
        (status = 204, description = "Word deleted successfully"),
        (status = 404, description = "Word not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Vocabulary"
)]
pub async fn delete_word(
    db_pool: web::Data<PgPool>,
    path: web::Path<uuid::Uuid>,
) -> Result<impl Responder, ActixError> {
    let vocabulary_repo = VocabularyDb::new(db_pool.as_ref());
    vocabulary_repo
        .get_word_by_id(*path)
        .await
        .map_err(DetailWordError::from)?
        .ok_or_else(DetailWordError::not_found)?;

    vocabulary_repo
        .delete_word(*path)
        .await
        .map_err(DetailWordError::from)?;

    Ok(HttpResponse::NoContent().finish())
}

