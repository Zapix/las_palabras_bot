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
