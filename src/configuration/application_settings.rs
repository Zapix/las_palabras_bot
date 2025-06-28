use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

impl ApplicationSettings {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_settings_creation() {
        let settings = ApplicationSettings::new("localhost".to_string(), 8080);
        assert_eq!(settings.host, "localhost");
        assert_eq!(settings.port, 8080);
        assert_eq!(settings.address(), "localhost:8080");
    }
}
