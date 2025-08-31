use crate::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use actix_web::{HttpResponse, Responder, error, web};
use sqlx::PgPool;

#[tracing::instrument(name = "detail_word", skip(db_pool))]
pub async fn get_word(
    db_pool: web::Data<PgPool>,
    word_id: web::Path<uuid::Uuid>,
) -> Result<impl Responder, actix_web::Error> {
    let vocabulary_repo = VocabularyDb::new(db_pool.as_ref());
    match vocabulary_repo
        .get_word_by_id(*word_id)
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot get word from db"))?
    {
        None => return Ok(HttpResponse::NotFound().body("Word not found")),
        Some(info) => Ok(HttpResponse::Ok().json(info)),
    }
}
