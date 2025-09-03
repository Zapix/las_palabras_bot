use actix_web::{Error, HttpResponse, Responder, error, web};
use sqlx::PgPool;
use tracing_attributes::instrument;

use crate::api::pagination::{DEFAULT_PAGE, PageQuery, Pagination};
use crate::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};

#[derive(serde::Deserialize, Debug)]
pub struct FilterQuery {
    pub is_verified: Option<bool>,
}

#[instrument(name = "list_words", skip(db_pool), err)]
pub async fn list_words(
    db_pool: web::Data<PgPool>,
    page_query: web::Query<PageQuery>,
    filter_query: web::Query<FilterQuery>,
) -> Result<impl Responder, Error> {
    let page = page_query.page.unwrap_or(0);
    let per_page = page_query.per_page.unwrap_or(DEFAULT_PAGE);

    let vocabulary_repo = VocabularyDb::new(db_pool.as_ref());
    let words_count = vocabulary_repo
        .count_words()
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot get words count from db"))?;

    let items = vocabulary_repo
        .list_word(page, per_page, filter_query.is_verified.into())
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot get words from db"))?;

    Ok(HttpResponse::Ok().json(Pagination {
        page,
        per_page,
        total: words_count as u64,
        items,
    }))
}
