use crate::domain::vocabulary::raw_word::RawWord;
use crate::domain::vocabulary::word::Word;
use crate::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use actix_web::{HttpResponse, Responder, error, web};
use sqlx::PgPool;

#[tracing::instrument(name = "create_word", skip(db_pool))]
#[utoipa::path(
    post,
    path = "/api/v1/vocabulary",
    request_body = RawWord,
    responses(
        (status = 201, description = "Word created successfully", body = Word),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Vocabulary")]
pub async fn create_word(
    db_pool: web::Data<PgPool>,
    raw_word: web::Json<RawWord>,
) -> Result<impl Responder, actix_web::Error> {
    let vocabulary_repo = VocabularyDb::new(db_pool.as_ref());
    let word = vocabulary_repo
        .create_word(raw_word.into_inner())
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot create word in db"))?;
    Ok(HttpResponse::Created().json(word))
}
