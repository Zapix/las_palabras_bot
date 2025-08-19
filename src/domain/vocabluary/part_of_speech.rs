use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection,
    Other,
}

impl PartOfSpeech {
    pub fn as_str(&self) -> &str {
        match self {
            PartOfSpeech::Noun => "noun",
            PartOfSpeech::Verb => "verb",
            PartOfSpeech::Adjective => "adjective",
            PartOfSpeech::Adverb => "adverb",
            PartOfSpeech::Pronoun => "pronoun",
            PartOfSpeech::Preposition => "preposition",
            PartOfSpeech::Conjunction => "conjunction",
            PartOfSpeech::Interjection => "interjection",
            PartOfSpeech::Other => "other",
        }
    }
}

impl From<&str> for PartOfSpeech {
    fn from(value: &str) -> Self {
        match value {
            "noun" => PartOfSpeech::Noun,
            "verb" => PartOfSpeech::Verb,
            "adjective" => PartOfSpeech::Adjective,
            "adverb" => PartOfSpeech::Adverb,
            "pronoun" => PartOfSpeech::Pronoun,
            "preposition" => PartOfSpeech::Preposition,
            "conjunction" => PartOfSpeech::Conjunction,
            "interjection" => PartOfSpeech::Interjection,
            _ => PartOfSpeech::Other,
        }
    }
}

impl From<String> for PartOfSpeech {
    fn from(value: String) -> Self {
        PartOfSpeech::from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_of_speech_as_str() {
        assert_eq!(PartOfSpeech::Noun.as_str(), "noun");
        assert_eq!(PartOfSpeech::Verb.as_str(), "verb");
        assert_eq!(PartOfSpeech::Adjective.as_str(), "adjective");
        assert_eq!(PartOfSpeech::Adverb.as_str(), "adverb");
        assert_eq!(PartOfSpeech::Pronoun.as_str(), "pronoun");
        assert_eq!(PartOfSpeech::Preposition.as_str(), "preposition");
        assert_eq!(PartOfSpeech::Conjunction.as_str(), "conjunction");
        assert_eq!(PartOfSpeech::Interjection.as_str(), "interjection");
        assert_eq!(PartOfSpeech::Other.as_str(), "other");
    }
    #[test]
    fn test_from_str() {
        assert_eq!(PartOfSpeech::from("noun"), PartOfSpeech::Noun);
        assert_eq!(PartOfSpeech::from("verb"), PartOfSpeech::Verb);
        assert_eq!(PartOfSpeech::from("adjective"), PartOfSpeech::Adjective);
        assert_eq!(PartOfSpeech::from("adverb"), PartOfSpeech::Adverb);
        assert_eq!(PartOfSpeech::from("pronoun"), PartOfSpeech::Pronoun);
        assert_eq!(PartOfSpeech::from("preposition"), PartOfSpeech::Preposition);
        assert_eq!(PartOfSpeech::from("conjunction"), PartOfSpeech::Conjunction);
        assert_eq!(
            PartOfSpeech::from("interjection"),
            PartOfSpeech::Interjection
        );
        assert_eq!(PartOfSpeech::from("unknown"), PartOfSpeech::Other);
    }
}
