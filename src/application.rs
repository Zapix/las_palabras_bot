use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;

use crate::api::{health, info, vocabulary};
use crate::configuration::Settings;
use crate::configuration::database_settings::DatabaseSettings;

pub struct Application {
    version: String,
    host: String,
    port: u16,
    server: Server,
}

impl Application {
    pub fn new(settings: Settings) -> Result<Self, anyhow::Error> {
        let address =
            TcpListener::bind((settings.application.host.clone(), settings.application.port))
                .map_err(|e| anyhow::anyhow!("Failed to bind to address: {}", e))?;

        let connection_pool = get_connection_pool(&settings.database);

        let version = settings.version.clone();
        let port = address.local_addr()?.port();
        let host = address.local_addr()?.ip().to_string();
        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .app_data(web::Data::new(settings.clone()))
                .app_data(web::Data::new(connection_pool.clone()))
                .route("/", web::get().to(|| async { "Hello, World!" }))
                .route("/health", web::get().to(health))
                .route("/info", web::get().to(info))
                .service(
                    web::scope("/api/v1").service(
                        web::resource("/vocabulary")
                            .route(web::get().to(vocabulary::list_words))
                            .route(web::post().to(vocabulary::create_word)),
                    ),
                )
            // Here you can add your routes, middleware, etc.
        })
        .listen(address)?
        .run();

        println!("Starting server at {}:{}", host, port);
        Ok(Self {
            version,
            host,
            port,
            server,
        })
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), anyhow::Error> {
        self.server
            .await
            .map_err(|e| anyhow::anyhow!("Server stopped with error: {}", e))
    }
}

pub fn get_connection_pool(db_settings: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy_with(db_settings.with_db_name())
}
