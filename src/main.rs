use actix_web::{web, App, HttpServer};

use las_palabras_bot::api::health;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello, World!" }))
            .route("/health", web::get().to(health))
        // Here you can add your routes, middleware, etc.
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
