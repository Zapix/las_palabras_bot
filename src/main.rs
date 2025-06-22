use las_palabras_bot::application::Application;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let app = Application::new("0.1.0".to_string(), "0.0.0.0".to_string(), 8080)
        .expect("Failed to create application");
    app.run_until_stopped().await?;

    Ok(())
}
