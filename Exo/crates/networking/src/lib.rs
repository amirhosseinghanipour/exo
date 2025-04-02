use reqwest::header::{HeaderValue, USER_AGENT};
use shared_types::{ExoError, Result, Url};
use log::info;

const FAKE_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) Exo/0.1.0";

static HTTP_CLIENT: tokio::sync::OnceCell<reqwest::Client> = tokio::sync::OnceCell::const_new();

async fn get_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| async {
        reqwest::Client::builder()
            .user_agent(HeaderValue::from_static(FAKE_USER_AGENT))
            .build()
            .expect("Failed to build reqwest client")
    }).await
}


pub async fn fetch_url(url: &Url) -> Result<String> {
    info!("Fetching URL: {}", url);
    let client = get_client().await;

    let response = client
        .get(url.clone())
        .send()
        .await
        .map_err(|e| ExoError::Network(format!("Request failed: {}", e)))?;

    if !response.status().is_success() {
        return Err(ExoError::Network(format!(
            "HTTP Error: {}",
            response.status()
        )));
    }

    let body = response
        .text()
        .await
        .map_err(|e| ExoError::Network(format!("Failed to read response body: {}", e)))?;

    info!("Successfully fetched {} bytes from {}", body.len(), url);
    Ok(body)
}
