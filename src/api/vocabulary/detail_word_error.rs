use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DetailWordError {
    #[error("Word not found")]
    NotFound,
    #[error("Database error: {0}")]
    InternalServerError(String),
}

impl DetailWordError {
    pub fn not_found() -> Self {
        DetailWordError::NotFound
    }
}

impl From<anyhow::Error> for DetailWordError {
    fn from(err: anyhow::Error) -> Self {
        DetailWordError::InternalServerError(err.to_string())
    }
}

impl ResponseError for DetailWordError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DetailWordError::NotFound => HttpResponse::NotFound().body(self.to_string()),
            DetailWordError::InternalServerError(_) => {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
    }
}
