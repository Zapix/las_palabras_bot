use super::super::GameType;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Variant {
    id: uuid::Uuid,
    text: String,
}

impl Variant {
    pub fn new(id: uuid::Uuid, text: String) -> Self {
        Self { id, text }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status")]
pub enum GameStatusOutput {
    Initialized,
    Asked {
        variants: Vec<Variant>,
        question: String,
    },
    Answered {
        is_correct: bool,
    },
    Ended,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameOutput {
    #[serde(with = "uuid::serde::simple")]
    pub id: uuid::Uuid,
    pub game_type: GameType,
    pub game_status: GameStatusOutput,
    pub questions_asked: i32,
    pub correct_answers: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
