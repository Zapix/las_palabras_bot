use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::regular_conjugacion::RegularConjugacion;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Perfecto {
    pub futuro: RegularConjugacion,
    pub pasado: RegularConjugacion,
    pub presente: RegularConjugacion,
    pub preterito: RegularConjugacion,
    pub condicional: RegularConjugacion,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Imperativo {
    pub negativo: RegularConjugacion,
    pub afirmativo: RegularConjugacion,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Indicativo {
    pub futuro: RegularConjugacion,
    pub presente: RegularConjugacion,
    pub preterito: RegularConjugacion,
    pub imperfecto: RegularConjugacion,
    pub condicional: RegularConjugacion,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Progresivo {
    pub futuro: RegularConjugacion,
    pub presente: RegularConjugacion,
    pub preterito: RegularConjugacion,
    pub imperfecto: RegularConjugacion,
    pub condicional: RegularConjugacion,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Subjuntivo {
    pub futuro: RegularConjugacion,
    pub presente: RegularConjugacion,
    pub imperfecto: RegularConjugacion,
    pub imperfecto2: RegularConjugacion,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PerfectoSubjuntivo {
    pub futuro: RegularConjugacion,
    pub presente: RegularConjugacion,
    pub pasado: RegularConjugacion,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Verb {
    #[serde(with = "uuid::serde::simple")]
    pub id: Uuid,
    pub verb: String,
    pub perfecto: Option<sqlx::types::Json<Perfecto>>,
    pub imperativo: Option<sqlx::types::Json<Imperativo>>,
    pub indicativo: Option<sqlx::types::Json<Indicativo>>,
    pub progresivo: Option<sqlx::types::Json<Progresivo>>,
    pub subjuntivo: Option<sqlx::types::Json<Subjuntivo>>,
    pub perfecto_subjuntivo: Option<sqlx::types::Json<PerfectoSubjuntivo>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
