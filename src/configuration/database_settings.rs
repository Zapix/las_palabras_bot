use secrecy::{Secret, ExposeSecret};
use serde::{Deserialize};
use serde_aux::prelude::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn new(username: String, password: String, port: u16, host: String, database_name: String, require_ssl: bool) -> Self {
        Self {
            username,
            password: Secret::new(password),
            port,
            host,
            database_name,
            require_ssl,
        }
    }

    pub fn without_db_name(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
            .ssl_mode(if self.require_ssl {
                PgSslMode::Require
            } else {
                PgSslMode::Disable
            })
    }

    pub fn with_db_name(&self) -> PgConnectOptions {
        self.without_db_name()
            .database(&self.database_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_settings_creation() {
        let settings = DatabaseSettings::new(
            "user".to_string(),
            "password".to_string(),
            5432,
            "localhost".to_string(),
            "test_db".to_string(),
            true,
        );
        assert_eq!(settings.username, "user");
        assert_eq!(settings.password.expose_secret(), "password");
        assert_eq!(settings.port, 5432);
        assert_eq!(settings.host, "localhost");
        assert_eq!(settings.database_name, "test_db");
        assert!(settings.require_ssl);
    }

    #[test]
    fn test_database_settings_without_db_name() {
        let settings = DatabaseSettings::new(
            "user".to_string(),
            "password".to_string(),
            5432,
            "localhost".to_string(),
            "test_db".to_string(),
            true,
        );
        let options = settings.without_db_name();
        assert_eq!(options.get_username(), "user");
        assert_eq!(options.get_host(), "localhost");
        assert_eq!(options.get_port(), 5432);
        assert!(options.get_database().is_none());
    }

    #[test]
    fn test_database_settings_with_db_name() {
        let settings = DatabaseSettings::new(
            "user".to_string(),
            "password".to_string(),
            5432,
            "localhost".to_string(),
            "test_db".to_string(),
            true,
        );
        let options = settings.with_db_name();
        assert_eq!(options.get_username(), "user");
        assert_eq!(options.get_database(), Some("test_db"));
        assert_eq!(options.get_host(), "localhost");
        assert_eq!(options.get_port(), 5432);
    }
}