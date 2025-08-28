use dotenv::dotenv;
use las_palabras_bot::application::get_connection_pool;
use las_palabras_bot::configuration::DatabaseSettings;
use log::info;
use secrecy::ExposeSecret;
use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;

const LOCALHOST_HOST: &str = "127.0.0.1";

pub struct TestApp {
    port: u16,
    api_client: reqwest::Client,
    db_pool: PgPool,
}

impl TestApp {
    pub fn new(port: u16, db_pool: PgPool) -> Result<Self, anyhow::Error> {
        let api_client = reqwest::Client::builder().build()?;
        Ok(Self {
            port,
            api_client,
            db_pool,
        })
    }

    pub fn api_client(&self) -> &reqwest::Client {
        &self.api_client
    }

    pub fn address(&self) -> String {
        format!("http://{}:{}", LOCALHOST_HOST, self.port)
    }

    pub fn db_pool(&mut self) -> &mut PgPool {
        &mut self.db_pool
    }
}

pub async fn configure_database(db_settings: &mut DatabaseSettings) -> Result<(), anyhow::Error> {
    let maintenance_settings = DatabaseSettings::new(
        db_settings.username.clone(),
        db_settings.password.expose_secret().to_string(),
        db_settings.port,
        db_settings.host.clone(),
        "postgres".to_string(), // Use the default postgres database for maintenance
        db_settings.require_ssl,
    );
    let mut connection = PgConnection::connect_with(&maintenance_settings.with_db_name())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to postgres: {}", e))?;
    let database_name = format!("test_{}", Uuid::new_v4().to_string().replace("-", "_"));
    sqlx::query(format!("CREATE DATABASE {};", database_name.as_str()).as_str())
        .execute(&mut connection)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create database: {}", e))?;
    db_settings.database_name = database_name;
    let mut connection = PgConnection::connect_with(&db_settings.with_db_name())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to new database: {}", e))?;
    sqlx::migrate!("./migrations")
        .run(&mut connection)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;
    Ok(())
}

pub async fn spawn_app() -> Result<TestApp, anyhow::Error> {
    dotenv().ok();
    let mut settings = las_palabras_bot::configuration::Settings::load()
        .map_err(|e| anyhow::anyhow!("Failed to load settings: {}", e))?;
    configure_database(&mut settings.database)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to configure database: {}", e))?;
    info!("Database configured with settings: {:?}", settings.database);
    let db_settings = settings.database.clone();
    settings.application.host = LOCALHOST_HOST.to_string();
    settings.application.port = 0; // Use 0 to let the OS assign a free port
    let app = las_palabras_bot::application::Application::new(settings)
        .map_err(|e| anyhow::anyhow!("Can not run server: {}", e))?;
    let port = app.port();
    tokio::spawn(app.run_until_stopped());

    let db_pool = get_connection_pool(&db_settings);
    let test_app = TestApp::new(port, db_pool)
        .map_err(|e| anyhow::anyhow!("Failed to create TestApp: {}", e))?;
    info!("Test application running at: {}", test_app.address());
    Ok(test_app)
}
