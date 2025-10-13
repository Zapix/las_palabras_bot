use actix_web::{Error, HttpResponse, Responder, error, web};
use sqlx::PgPool;
use tracing_attributes::instrument;

use crate::api::pagination::{DEFAULT_PAGE, PageQuery, Pagination};
use crate::domain::verbs::light_verb::LightVerb;
use crate::domain::verbs::repository::{VerbsDb, VerbsRepository};

#[instrument(name = "list_words", skip(db_pool), err)]
#[utoipa::path(
    get,
    path = "/api/v1/verbs",
    params(
        ("page" = Option<u64>, Query, description = "Page number, starting from 0"),
        ("per_page" = Option<u64>, Query, description = "Number of items per page"),
    ),
    responses(
        (status = 200, description = "List of verbs", body = Pagination<LightVerb>),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Verbs"
)]
pub async fn list_verbs(
    db_pool: web::Data<PgPool>,
    page_query: web::Query<PageQuery>,
) -> Result<impl Responder, Error> {
    let page = page_query.page.unwrap_or(0);
    let per_page = page_query.per_page.unwrap_or(DEFAULT_PAGE);

    let verbs_repo = VerbsDb::new(db_pool.as_ref());
    let verbs_count = verbs_repo
        .count_verbs()
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot get verbs count from db"))?;

    let items = verbs_repo
        .list_verbs(page, per_page)
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot get verbs from db"))?;

    Ok(HttpResponse::Ok().json(Pagination {
        page,
        per_page,
        total: verbs_count as u64,
        items,
    }))
}
