use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use crate::api::{health, info};
use crate::configuration::Settings;

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

        let version = settings.version.clone();
        let port = address.local_addr().unwrap().port();
        let host = address.local_addr().unwrap().ip().to_string();
        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(settings.clone()))
                .route("/", web::get().to(|| async { "Hello, World!" }))
                .route("/health", web::get().to(health))
                .route("/info", web::get().to(info))
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
