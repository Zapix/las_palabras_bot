use dotenv::dotenv;
const LOCALHOST_HOST: &str = "127.0.0.1";

pub struct TestApp {
    port: u16,
    api_client: reqwest::Client,
}

impl TestApp {
    pub fn new(port: u16) -> Result<Self, anyhow::Error> {
        let api_client = reqwest::Client::builder().build()?;
        Ok(Self { port, api_client })
    }

    pub fn api_client(&self) -> &reqwest::Client {
        &self.api_client
    }

    pub fn address(&self) -> String {
        format!("http://{}:{}", LOCALHOST_HOST, self.port)
    }
}

pub async fn spawn_app() -> Result<TestApp, anyhow::Error> {
    dotenv().ok();
    let mut settings = las_palabras_bot::configuration::Settings::load()
        .map_err(|e| anyhow::anyhow!("Failed to load settings: {}", e))?;
    settings.application.host = LOCALHOST_HOST.to_string();
    settings.application.port = 0; // Use 0 to let the OS assign a free port
    let app = las_palabras_bot::application::Application::new(settings)
        .map_err(|e| anyhow::anyhow!("Can not run server: {}", e))?;
    let port = app.port();
    tokio::spawn(app.run_until_stopped());

    let test_app =
        TestApp::new(port).map_err(|e| anyhow::anyhow!("Failed to create TestApp: {}", e))?;
    println!("Test application running at: {}", test_app.address());
    Ok(test_app)
}
