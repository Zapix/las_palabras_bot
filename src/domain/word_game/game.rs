use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use super::game_status::GameStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GameType {
    #[default]
    RussianToSpanish,
    SpanishToRussian,
}

impl GameType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GameType::RussianToSpanish => "RussianToSpanish",
            GameType::SpanishToRussian => "SpanishToRussian",
        }
    }
}

impl TryFrom<&str> for GameType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "RussianToSpanish" => Ok(GameType::RussianToSpanish),
            "SpanishToRussian" => Ok(GameType::SpanishToRussian),
            _ => Err(()),
        }
    }
}

impl From<GameType> for String {
    fn from(value: GameType) -> Self {
        value.as_str().to_string()
    }
}

impl From<String> for GameType {
    fn from(value: String) -> Self {
        GameType::try_from(value.as_str()).unwrap_or_default()
    }
}

impl Display for GameType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    #[serde(with = "uuid::serde::simple")]
    pub id: uuid::Uuid,
    pub game_type: GameType,
    pub game_status: Option<sqlx::types::Json<GameStatus>>,
    pub questions_asked: i32,
    pub correct_answers: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
