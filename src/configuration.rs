use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEnvironment {
    Development,
    Production,
}

impl AppEnvironment {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppEnvironment::Development => "development",
            AppEnvironment::Production => "production",
        }
    }
}

impl From<String> for AppEnvironment {
    fn from(env: String) -> Self {
        match env.as_str() {
            "development" => AppEnvironment::Development,
            "production" => AppEnvironment::Production,
            _ => panic!("Unknown environment: {}", env),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramSettings {
    pub bot_token: String,
}

impl TelegramSettings {
    pub fn new(bot_token: String) -> Self {
        Self { bot_token }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub version: String,
    environment: String,
    telegram: TelegramSettings,
}

impl Settings {
    pub fn new(version: String, environment: String, telegram: TelegramSettings) -> Self {
        Self {
            version,
            environment,
            telegram,
        }
    }

    pub fn load() -> Result<Self, config::ConfigError> {
        let config_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("config");

        let environtment =
            std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        let config_file = format!("{}.yaml", environtment);

        let settings = Config::builder()
            .add_source(config::File::from(config_dir.join("base.yaml")))
            .add_source(config::File::from(config_dir.join(&config_file)))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        settings.try_deserialize::<Settings>()
    }

    pub fn environment(&self) -> AppEnvironment {
        AppEnvironment::from(self.environment.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_creation() {
        let config = Settings::new(
            "1.0.0".to_string(),
            "development".to_string(),
            TelegramSettings::new("your_bot_token".to_string()),
        );
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.environment, "development");
    }

    #[test]
    fn test_is_development() {
        let config = Settings::new(
            "1.0.0".to_string(),
            "development".to_string(),
            TelegramSettings::new("your_bot_token".to_string()),
        );
        assert_eq!(config.environment(), AppEnvironment::Development);
    }

    #[test]
    fn test_is_production() {
        let config = Settings::new(
            "1.0.0".to_string(),
            "production".to_string(),
            TelegramSettings::new("your_bot_token".to_string()),
        );
        assert_eq!(config.environment(), AppEnvironment::Production);
    }

    #[test]
    fn test_load_configuration() {
        let config = Settings::load();
        println!("Loaded configuration: {:?}", config);
        assert!(config.is_ok(), "Failed to load configuration");
        let settings = config.unwrap();
        assert!(!settings.version.is_empty());
        assert!(!settings.environment.is_empty());
    }
}
