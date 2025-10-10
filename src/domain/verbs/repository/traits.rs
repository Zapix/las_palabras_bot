use anyhow::Result;
use std::future::Future;

use super::super::light_verb::LightVerb;
use super::super::raw_verb::RawVerb;
use super::super::verb::Verb;

pub trait VerbsRepository {
    fn add_verb(&self, verb: RawVerb) -> impl Future<Output = Result<Verb>> + Send;
    fn add_batch_verbs(
        &self,
        verbs: Vec<RawVerb>,
    ) -> impl Future<Output = Result<Vec<Verb>>> + Send;

    fn list_verbs(
        &self,
        page: u64,
        per_page: u64,
    ) -> impl Future<Output = Result<Vec<LightVerb>>> + Send;

    fn count_words(&self) -> impl Future<Output = Result<i64>> + Send;
    fn get_verb_by_id(&self, id: uuid::Uuid) -> impl Future<Output = Result<Option<Verb>>> + Send;
}
