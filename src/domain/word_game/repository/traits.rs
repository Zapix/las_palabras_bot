use super::super::game::{Game, GameType};
use super::GameRepositoryError;
use anyhow::Result;
use std::future::Future;

pub trait GameTrait {
    fn new_game(&mut self, game_type: GameType) -> impl Future<Output = Result<Game>> + Send;

    fn load_game_by_id(
        &mut self,
        id: uuid::Uuid,
    ) -> impl Future<Output = Result<Option<Game>>> + Send;

    fn ask_question(
        &mut self,
        game_id: uuid::Uuid,
    ) -> impl Future<Output = Result<Game, GameRepositoryError>> + Send;

    /*
    fn answer_question(
        &mut self,
        game_id: uuid::Uuid,
        answer: uuid::Uuid,
    ) -> impl Future<Output = Result<Game>> + Send;

    fn end_game(&mut self, game_id: uuid::Uuid) -> impl Future<Output = Result<Game>> + Send;
    */
}
