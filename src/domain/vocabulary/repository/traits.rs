use anyhow::Result;
use std::future::Future;

use crate::domain::vocabulary::raw_word::RawWord;
use crate::domain::vocabulary::word::Word;

pub trait VocabularyTrait {
    fn create_word(&self, raw_word: RawWord) -> impl Future<Output = Result<Word>> + Send;
    fn create_batch_words(
        &self,
        raw_words: Vec<RawWord>,
    ) -> impl Future<Output = Result<Vec<Word>>> + Send;
    fn get_word_by_id(&self, id: uuid::Uuid) -> impl Future<Output = Result<Option<Word>>> + Send;
    fn verify_word(&self, id: uuid::Uuid) -> impl Future<Output = Result<Word>> + Send;
    fn update_word(
        &self,
        id: uuid::Uuid,
        raw_word: RawWord,
    ) -> impl Future<Output = Result<Word>> + Send;
    fn delete_word(&self, id: uuid::Uuid) -> impl Future<Output = Result<()>> + Send;
}
