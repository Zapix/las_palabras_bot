use anyhow::Result;

use super::filters::IsVerifiedFilter;
use crate::domain::vocabulary::raw_word::RawWord;
use crate::domain::vocabulary::word::Word;

#[async_trait::async_trait]
pub trait VocabularyTrait {
    async fn create_word(&self, raw_word: RawWord) -> Result<Word>;

    async fn create_batch_words(&self, raw_words: Vec<RawWord>) -> Result<Vec<Word>>;

    async fn list_word(
        &self,
        page: u64,
        per_page: u64,
        filter: IsVerifiedFilter,
    ) -> Result<Vec<Word>>;

    async fn list_random_words(&self, limit: u64) -> Result<Vec<Word>>;

    async fn list_word_by_ids(&self, ids: &[uuid::Uuid]) -> Result<Vec<Word>>;

    async fn count_words(&self) -> Result<i64>;

    async fn get_word_by_id(&self, id: uuid::Uuid) -> Result<Option<Word>>;

    async fn verify_word(&self, id: uuid::Uuid) -> Result<Word>;

    async fn update_word(&self, id: uuid::Uuid, raw_word: RawWord) -> Result<Word>;

    async fn delete_word(&self, id: uuid::Uuid) -> Result<()>;
}
