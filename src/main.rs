use dotenv::dotenv;

use las_palabras_bot::application::Application;
use las_palabras_bot::configuration::Settings;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let settings = Settings::load().expect("Failed to load settings");
    let app = Application::new(settings).expect("Failed to create application");
    app.run_until_stopped().await?;

    Ok(())
}
