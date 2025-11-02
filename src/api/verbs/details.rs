use actix_web::{Error, HttpResponse, Responder, error, web};
use sqlx::PgPool;
use tracing_attributes::instrument;

use super::web_verb::WebVerb;
use crate::domain::verbs::repository::{VerbsDb, VerbsRepository};

#[instrument(name = "details_verb", skip(db_pool), err)]
#[utoipa::path(
    get,
    path = "/api/v1/verbs/{id}",
    params(
        ("id" = String, Path, description = "ID of the verb to retrieve as uuid"),
    ),
    responses(
        (status = 200, description = "Details of the verb", body = WebVerb),
        (status = 404, description = "Verb not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Verbs"
)]
pub async fn get_verb(
    db_pool: web::Data<PgPool>,
    veb_id: web::Path<uuid::Uuid>,
) -> Result<impl Responder, Error> {
    let verbs_repo = VerbsDb::new(db_pool.as_ref());
    let verb = verbs_repo
        .get_verb_by_id(*veb_id)
        .await
        .map_err(|_e| error::ErrorInternalServerError("Cannot get verb from db"))?
        .ok_or_else(|| error::ErrorNotFound("Verb not found"))?;

    let web_verb: WebVerb = verb.into();

    Ok(HttpResponse::Ok().json(web_verb))
}
