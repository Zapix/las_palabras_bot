use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct LightVerb {
    #[serde(with = "uuid::serde::simple")]
    pub id: Uuid,
    pub verb: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}
