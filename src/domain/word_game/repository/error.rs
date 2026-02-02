use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum GameRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Game not found")]
    GameNotFound,
    #[error("Invalid game state transition")]
    InvalidGameStateTransition,
}
