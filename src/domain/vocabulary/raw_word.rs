use serde::{Deserialize, Serialize};

use super::part_of_speech::PartOfSpeech;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawWord {
    pub spanish: String,
    pub russian: String,
    pub part_of_speech: PartOfSpeech,
}
