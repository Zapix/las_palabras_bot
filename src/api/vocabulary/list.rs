use actix_web::{Error, HttpResponse, Responder, error, web};
use sqlx::PgPool;
use tracing_attributes::instrument;

use crate::api::pagination::{DEFAULT_PAGE, PageQuery, Pagination};
use crate::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};
use crate::domain::vocabulary::word::Word;

#[derive(serde::Deserialize, Debug)]
pub struct FilterQuery {
    pub is_verified: Option<bool>,
}

#[instrument(name = "list_words", skip(db_pool), err)]
#[utoipa::path(
    get,
    path = "/api/v1/vocabulary",
    params(
        ("page" = Option<u64>, Query, description = "Page number, starting from 0"),
        ("per_page" = Option<u64>, Query, description = "Number of items per page"),
        ("is_verified" = Option<bool>, Query, description = "Filter by verification status"),
    ),
    responses(
        (status = 200, description = "List of words", body = Pagination<Word>),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Vocabulary"
)]
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
