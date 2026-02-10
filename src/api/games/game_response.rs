use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::word_game::converter::{GameOutput, GameStatusOutput};

#[derive(Debug, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
    #[serde(with = "uuid::serde::simple")]
    pub id: Uuid,
    pub game_type: String,
    pub status: String,
    pub questions_asked: i32,
    pub correct_answers: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<GameOutput> for GameResponse {
    fn from(value: GameOutput) -> Self {
        let status = match value.game_status {
            GameStatusOutput::Initialized => "Initialized",
            GameStatusOutput::Asked { .. } => "Asked",
            GameStatusOutput::Answered { .. } => "Answered",
            GameStatusOutput::Ended => "Ended",
        }
        .to_string();

        Self {
            id: value.id,
            game_type: value.game_type.to_string(),
            status,
            questions_asked: value.questions_asked,
            correct_answers: value.correct_answers,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime};
    use uuid::Uuid;

    use super::GameResponse;
    use crate::domain::word_game::GameType;
    use crate::domain::word_game::converter::{GameOutput, GameStatusOutput, Variant};

    fn dt(year: i32, month: u32, day: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .expect("valid date")
            .and_hms_opt(12, 0, 0)
            .expect("valid time")
    }

    fn game_output_with_status(game_status: GameStatusOutput) -> GameOutput {
        GameOutput {
            id: Uuid::new_v4(),
            game_type: GameType::RussianToSpanish,
            game_status,
            questions_asked: 3,
            correct_answers: 2,
            created_at: dt(2026, 2, 10),
            updated_at: dt(2026, 2, 11),
        }
    }

    #[test]
    fn from_game_output_maps_initialized_status() {
        let output = game_output_with_status(GameStatusOutput::Initialized);

        let response = GameResponse::from(output.clone());

        assert_eq!(response.id, output.id);
        assert_eq!(response.game_type, "RussianToSpanish");
        assert_eq!(response.status, "Initialized");
        assert_eq!(response.questions_asked, output.questions_asked);
        assert_eq!(response.correct_answers, output.correct_answers);
        assert_eq!(response.created_at, output.created_at);
        assert_eq!(response.updated_at, output.updated_at);
    }

    #[test]
    fn from_game_output_maps_asked_status() {
        let output = game_output_with_status(GameStatusOutput::Asked {
            variants: vec![Variant::new(Uuid::new_v4(), "hola".to_string())],
            question: "привет".to_string(),
        });

        let response = GameResponse::from(output);

        assert_eq!(response.status, "Asked");
    }

    #[test]
    fn from_game_output_maps_answered_status() {
        let output = game_output_with_status(GameStatusOutput::Answered { is_correct: true });

        let response = GameResponse::from(output);

        assert_eq!(response.status, "Answered");
    }

    #[test]
    fn from_game_output_maps_ended_status() {
        let output = game_output_with_status(GameStatusOutput::Ended);

        let response = GameResponse::from(output);

        assert_eq!(response.status, "Ended");
    }
}
