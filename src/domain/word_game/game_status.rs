use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status")]
pub enum GameStatus {
    Initialized,
    Asked {
        word_ids: Vec<uuid::Uuid>,
        #[serde(skip)]
        correct_word_id: uuid::Uuid,
    },
    Answered {
        word_ids: Vec<uuid::Uuid>,
        correct_word_id: uuid::Uuid,
        is_correct: bool,
    },
    Ended,
}
