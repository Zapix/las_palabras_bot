use actix_web::web;
use anyhow::Result;
use sqlx::PgPool;
use tracing::info;
use tracing_attributes::instrument;

pub mod info;
pub mod pagination;
pub mod verbs;
pub mod vocabulary;
pub use info::info;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = String),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Health"
)]
#[instrument(name = "health_check", skip(db_pool), err)]
pub async fn health(db_pool: web::Data<PgPool>) -> Result<String, actix_web::Error> {
    // Check if the database connection is healthy
    info!("Checking database connection health");
    sqlx::query("SELECT 1")
        .fetch_one(db_pool.get_ref())
        .await
        .map_err(|e| anyhow::anyhow!("Database connection error: {}", e))
        .map_err(actix_web::error::ErrorInternalServerError)?;
    info!("Successfully connected to the database");

    Ok("OK".to_string())
}
