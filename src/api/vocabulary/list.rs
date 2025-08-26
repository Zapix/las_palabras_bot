use actix_web::{Error, web};
use sqlx::PgPool;
use tracing_attributes::instrument;

use crate::api::pagination::PageQuery;

#[instrument(name = "list_words", skip(_db_pool), err)]
pub async fn list_words(
    _db_pool: web::Data<PgPool>,
    page_query: web::Query<PageQuery>,
) -> Result<String, Error> {
    let page = page_query.page.unwrap_or(0);
    let per_page = page_query.per_page.unwrap_or(20);

    let information = format!("Listing words - Page: {}, Per Page: {}", page, per_page);

    Ok(information)
}
