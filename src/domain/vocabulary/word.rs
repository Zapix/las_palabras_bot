use super::part_of_speech::PartOfSpeech;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Word {
    pub id: Uuid,
    pub spanish: String,
    pub russian: String,
    pub part_of_speech: PartOfSpeech,
    pub is_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
