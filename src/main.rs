use dotenv::dotenv;

use las_palabras_bot::application::Application;
use las_palabras_bot::configuration::Settings;
use las_palabras_bot::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let app_name = env!("CARGO_PKG_NAME");
    let subscriber = get_subscriber(app_name.into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = Settings::load().expect("Failed to load settings");
    let app = Application::new(settings).expect("Failed to create application");
    app.run_until_stopped().await?;

    Ok(())
}
