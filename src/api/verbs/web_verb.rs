use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::verbs::verb::{
    Imperativo, Indicativo, Perfecto, PerfectoSubjuntivo, Progresivo, Subjuntivo, Verb,
};

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct WebVerb {
    id: uuid::Uuid,
    verb: String,
    perfecto: Option<Perfecto>,
    imperativo: Option<Imperativo>,
    indicativo: Option<Indicativo>,
    progresivo: Option<Progresivo>,
    subjuntivo: Option<Subjuntivo>,
    perfecto_subjuntivo: Option<PerfectoSubjuntivo>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<Verb> for WebVerb {
    fn from(verb: Verb) -> Self {
        WebVerb {
            id: verb.id,
            verb: verb.verb,
            perfecto: verb.perfecto.map(|p| p.0),
            imperativo: verb.imperativo.map(|i| i.0),
            indicativo: verb.indicativo.map(|i| i.0),
            progresivo: verb.progresivo.map(|p| p.0),
            subjuntivo: verb.subjuntivo.map(|s| s.0),
            perfecto_subjuntivo: verb.perfecto_subjuntivo.map(|p| p.0),
            created_at: verb.created_at,
            updated_at: verb.updated_at,
        }
    }
}
