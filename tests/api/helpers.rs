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
    let app = las_palabras_bot::application::Application::new(
        "0.1.0".to_string(),
        LOCALHOST_HOST.to_string(),
        0,
    )
    .map_err(|e| anyhow::anyhow!("Can not run server: {}", e))?;
    let port = app.port();
    let _ = tokio::spawn(app.run_until_stopped());

    TestApp::new(port)
        .map_err(|e| anyhow::anyhow!("Failed to create TestApp: {}", e))
        .map(|test_app| {
            println!("Test application running at: {}", test_app.address());
            test_app
        })
}
