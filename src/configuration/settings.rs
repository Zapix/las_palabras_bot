use config::Config;
use serde::{Deserialize};

use super::app_enviroment::AppEnvironment;
use super::application_settings::ApplicationSettings;
use super::telegram_settings::TelegramSettings;
use super::database_settings::DatabaseSettings;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub version: String,
    environment: String,
    pub application: ApplicationSettings,
    pub telegram: TelegramSettings,
    pub database: DatabaseSettings
}

impl Settings {
    pub fn new(
        version: String,
        environment: String,
        application: ApplicationSettings,
        telegram: TelegramSettings,
        database: DatabaseSettings,
    ) -> Self {
        Self {
            version,
            environment,
            telegram,
            application,
            database,
        }
    }

    pub fn load() -> Result<Self, config::ConfigError> {
        let config_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("config");

        let environment =
            std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        let version = env!("CARGO_PKG_VERSION");
        let config_file = format!("{}.yaml", environment);

        let settings = Config::builder()
            .add_source(config::File::from(config_dir.join("base.yaml")))
            .add_source(config::File::from(config_dir.join(&config_file)))
            .add_source(config::Environment::with_prefix("APP"))
            .set_override_option("environment", Some(environment))?
            .set_override_option("version", Some(version))?
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
            ApplicationSettings::default(),
            TelegramSettings::new("your_bot_token".to_string()),
            DatabaseSettings::new(
                "user".to_string(),
                "password".to_string(),
                5432,
                "localhost".to_string(),
                "test_db".to_string(),
                true,
            ),
        );
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.environment, "development");
    }

    #[test]
    fn test_is_development() {
        let config = Settings::new(
            "1.0.0".to_string(),
            "development".to_string(),
            ApplicationSettings::default(),
            TelegramSettings::new("your_bot_token".to_string()),
            DatabaseSettings::new(
                "user".to_string(),
                "password".to_string(),
                5432,
                "localhost".to_string(),
                "test_db".to_string(),
                true,
            ),
        );
        assert_eq!(config.environment(), AppEnvironment::Development);
    }

    #[test]
    fn test_is_production() {
        let config = Settings::new(
            "1.0.0".to_string(),
            "production".to_string(),
            ApplicationSettings::default(),
            TelegramSettings::new("your_bot_token".to_string()),
            DatabaseSettings::new(
                "user".to_string(),
                "password".to_string(),
                5432,
                "localhost".to_string(),
                "test_db".to_string(),
                true,
            ),
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
