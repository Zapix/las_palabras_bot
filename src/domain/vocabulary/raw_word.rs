use serde::{Deserialize, Serialize};

use super::part_of_speech::PartOfSpeech;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawWord {
    pub spanish: String,
    pub russian: String,
    pub part_of_speech: PartOfSpeech,
    #[serde(default)]
    pub is_verified: Option<bool>,
}

impl Default for RawWord {
    fn default() -> Self {
        Self {
            spanish: String::new(),
            russian: String::new(),
            part_of_speech: PartOfSpeech::Noun,
            is_verified: Some(false),
        }
    }
}
