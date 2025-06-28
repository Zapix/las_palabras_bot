use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramSettings {
    pub bot_token: String,
}

impl TelegramSettings {
    pub fn new(bot_token: String) -> Self {
        Self { bot_token }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telegram_settings_creation() {
        let settings = TelegramSettings::new("your_bot_token".to_string());
        assert_eq!(settings.bot_token, "your_bot_token");
    }
}
