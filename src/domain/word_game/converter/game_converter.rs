use super::super::GameType;
use super::super::game::Game;
use super::super::game_status::GameStatus;
use super::output::{GameOutput, GameStatusOutput, Variant};
use crate::domain::vocabulary::repository::VocabularyTrait;
use anyhow::anyhow;

pub struct Converter<'a> {
    vocabulary: &'a dyn VocabularyTrait,
}

impl<'a> Converter<'a> {
    pub fn new(vocabulary: &'a dyn VocabularyTrait) -> Self {
        Self { vocabulary }
    }

    pub async fn convert_game_to_output(&self, game: &Game) -> anyhow::Result<GameOutput> {
        let game_status = game
            .game_status
            .as_ref()
            .map(|status| status.0.clone())
            .ok_or_else(|| anyhow!("game status is missing for game {}", game.id))?;

        let game_status = match game_status {
            GameStatus::Initialized => GameStatusOutput::Initialized,
            GameStatus::Asked {
                word_ids,
                correct_word_id,
            } => {
                let words = self.vocabulary.list_word_by_ids(&word_ids).await?;
                let correct_word = words
                    .iter()
                    .find(|word| word.id == correct_word_id)
                    .ok_or_else(|| anyhow!("correct word {} not found", correct_word_id))?;

                let question = match game.game_type {
                    GameType::RussianToSpanish => correct_word.russian.clone(),
                    GameType::SpanishToRussian => correct_word.spanish.clone(),
                };

                let variants = words
                    .into_iter()
                    .map(|word| {
                        let text = match game.game_type {
                            GameType::RussianToSpanish => word.spanish,
                            GameType::SpanishToRussian => word.russian,
                        };
                        Variant::new(word.id, text)
                    })
                    .collect();

                GameStatusOutput::Asked { variants, question }
            }
            GameStatus::Answered { is_correct, .. } => GameStatusOutput::Answered { is_correct },
            GameStatus::Ended => GameStatusOutput::Ended,
        };

        Ok(GameOutput {
            id: game.id,
            game_type: game.game_type.clone(),
            game_status,
            questions_asked: game.questions_asked,
            correct_answers: game.correct_answers,
            created_at: game.created_at,
            updated_at: game.updated_at,
        })
    }
}
