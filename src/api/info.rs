use actix_web::{web::Data, HttpResponse};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_attributes::instrument;

use crate::configuration::Settings;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Info {
    version: String,
    environment: String,
}

impl TryFrom<Data<Settings>> for Info {
    type Error = String;

    fn try_from(settings: Data<Settings>) -> Result<Self, Self::Error> {
        Ok(Self {
            version: settings.version.clone(),
            environment: settings.environment().as_str().to_string(),
        })
    }
}

#[instrument(err, skip(settings), fields(version = %settings.version))]
pub async fn info(settings: Data<Settings>) -> Result<HttpResponse, actix_web::Error> {
    info!(version = %settings.version, "print info about");
    let info =
        Info::try_from(settings).map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(info))
}
