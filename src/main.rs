mod models;
mod config;
mod tui;

use reqwest::Client;
use std::time::{Duration, Instant};
use models::{Model, get_models};
use config::{load_config, get_api_key};
use tui::{App, run_tui};

#[derive(Debug, Clone)]
pub struct PingResult {
    pub model_id: String,
    pub latency: Option<u128>,
    pub status: String,
}

async fn ping_model(client: &Client, model: &Model, api_key: Option<String>) -> PingResult {
    let start = Instant::now();
    let provider_url = match model.provider_key.as_str() {
        "nvidia" => "https://integrate.api.nvidia.com/v1/chat/completions",
        "groq" => "https://api.groq.com/openai/v1/chat/completions",
        // Add more providers here
        _ => "https://api.openai.com/v1/chat/completions",
    };

    if api_key.is_none() {
        return PingResult {
            model_id: model.id.clone(),
            latency: None,
            status: "Missing API Key".to_string(),
        };
    }

    let payload = serde_json::json!({
        "model": model.id,
        "messages": [{"role": "user", "content": "ping"}],
        "max_tokens": 1
    });

    let response = client
        .post(provider_url)
        .header("Authorization", format!("Bearer {}", api_key.unwrap()))
        .json(&payload)
        .timeout(Duration::from_secs(15))
        .send()
        .await;

    match response {
        Ok(res) if res.status().is_success() => PingResult {
            model_id: model.id.clone(),
            latency: Some(start.elapsed().as_millis()),
            status: "Success".to_string(),
        },
        Ok(res) => PingResult {
            model_id: model.id.clone(),
            latency: None,
            status: format!("Error: {}", res.status()),
        },
        Err(e) => PingResult {
            model_id: model.id.clone(),
            latency: None,
            status: format!("Failed: {}", e),
        },
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    let models = get_models();
    let client = Client::new();
    
    let app = App::new(models.clone());
    let tx = app.tx.clone();

    // Spawn pings in background tasks
    for model in models {
        let client_clone = client.clone();
        let tx_clone = tx.clone();
        let api_key = get_api_key(&config, &model.provider_key);
        
        tokio::spawn(async move {
            let result = ping_model(&client_clone, &model, api_key).await;
            let _ = tx_clone.send(result);
        });
    }

    run_tui(app)
}
