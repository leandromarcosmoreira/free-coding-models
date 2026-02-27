mod models;
mod config;
mod tui;
mod cli;

use clap::Parser;
use reqwest::Client;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use models::{Model, get_models, PingResult};
use config::{load_config, get_api_key};
use tui::{App, run_tui};
use cli::Cli;

#[derive(Debug, Clone)]
pub struct ModelStats {
    pub model: Model,
    pub avg_latency: f64,
    pub uptime: f64,
    pub success_count: usize,
    pub total_count: usize,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub ping_history: Vec<PingResult>,
}

async fn ping_model(client: &Client, model: &Model, api_key: Option<String>) -> PingResult {
    let start = Instant::now();
    
    let provider_url = match model.provider_key.as_str() {
        "nvidia" => "https://integrate.api.nvidia.com/v1/chat/completions",
        "groq" => "https://api.groq.com/openai/v1/chat/completions",
        "openrouter" => "https://openrouter.ai/api/v1/chat/completions",
        _ => "https://api.openai.com/v1/chat/completions",
    };

    if api_key.is_none() {
        return PingResult::error(model.id.clone(), "Missing API Key".to_string());
    }

    let payload = serde_json::json!({
        "model": model.id,
        "messages": [{"role": "user", "content": "ping"}],
        "max_tokens": 1
    });

    let response = client
        .post(provider_url)
        .header("Authorization", format!("Bearer {}", api_key.unwrap()))
        .header("Content-Type", "application/json")
        .json(&payload)
        .timeout(Duration::from_secs(15))
        .send()
        .await;

    match response {
        Ok(res) if res.status().is_success() => {
            PingResult::success(model.id.clone(), start.elapsed().as_millis())
        },
        Ok(res) => {
            PingResult::error(model.id.clone(), format!("Error: {}", res.status()))
        },
        Err(e) => {
            PingResult::error(model.id.clone(), format!("Failed: {}", e))
        },
    }
}

async fn ping_all_models(models: &[Model], config: &config::Config) -> HashMap<String, ModelStats> {
    let client = Client::new();
    let mut stats = HashMap::new();

    // Initialize stats for all models
    for model in models {
        stats.insert(model.id.clone(), ModelStats {
            model: model.clone(),
            avg_latency: 0.0,
            uptime: 0.0,
            success_count: 0,
            total_count: 0,
            last_seen: chrono::Utc::now(),
            ping_history: Vec::new(),
        });
    }

    // Ping all models in parallel
    let mut tasks = Vec::new();
    for model in models {
        let client_clone = client.clone();
        let model_clone = model.clone();
        let api_key = get_api_key(config, &model.provider_key);
        
        let task = tokio::spawn(async move {
            ping_model(&client_clone, &model_clone, api_key).await
        });
        tasks.push(task);
    }

    // Wait for all pings to complete
    for task in tasks {
        if let Ok(result) = task.await {
            if let Some(stats_entry) = stats.get_mut(&result.model_id) {
                stats_entry.total_count += 1;
                stats_entry.ping_history.push(result.clone());
                
                if let Some(latency) = result.latency {
                    stats_entry.success_count += 1;
                    stats_entry.last_seen = result.timestamp;
                    
                    // Calculate average latency
                    let total_latency: f64 = stats_entry.ping_history
                        .iter()
                        .filter_map(|p| p.latency)
                        .map(|l| l as f64)
                        .sum();
                    stats_entry.avg_latency = total_latency / stats_entry.success_count as f64;
                }
                
                // Calculate uptime
                stats_entry.uptime = (stats_entry.success_count as f64 / stats_entry.total_count as f64) * 100.0;
            }
        }
    }

    stats
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter(if cli.verbose {
            "debug"
        } else {
            "info"
        })
        .init();

    let config = load_config();
    let models = get_models();

    // Handle commands
    if let Some(command) = &cli.command {
        match command {
            cli::Commands::Config { action } => {
                handle_config_command(action, &config).await?;
                return Ok(());
            },
            cli::Commands::List { provider, tier } => {
                handle_list_command(&models, provider, tier).await?;
                return Ok(());
            },
            cli::Commands::Test { model_id, count } => {
                handle_test_command(model_id, *count, &config).await?;
                return Ok(());
            },
            cli::Commands::Version => {
                println!("free-coding-models {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            },
        }
    }

    // Run the main TUI application
    let stats = ping_all_models(&models, &config).await;
    let app = App::new(models, stats, cli);

    run_tui(app).await?;

    Ok(())
}

async fn handle_config_command(action: &cli::ConfigAction, config: &config::Config) -> anyhow::Result<()> {
    match action {
        cli::ConfigAction::SetKey { provider, key } => {
            println!("Setting API key for provider: {}", provider);
            // Implementation would update config
        },
        cli::ConfigAction::RemoveKey { provider } => {
            println!("Removing API key for provider: {}", provider);
            // Implementation would remove key from config
        },
        cli::ConfigAction::ListKeys => {
            println!("Listing configured API keys...");
            // Implementation would list all keys
        },
        cli::ConfigAction::Reset => {
            println!("Resetting all configuration...");
            // Implementation would reset config
        },
        cli::ConfigAction::Show => {
            println!("Current configuration:");
            // Implementation would show current config
        },
    }
    Ok(())
}

async fn handle_list_command(models: &[Model], provider: &Option<String>, tier: &Option<String>) -> anyhow::Result<()> {
    let filtered_models: Vec<&Model> = models.iter()
        .filter(|m| {
            if let Some(p) = provider {
                m.provider_key == *p
            } else {
                true
            }
        })
        .filter(|m| {
            if let Some(t) = tier {
                m.tier == *t
            } else {
                true
            }
        })
        .collect();

    println!("Available models:");
    for model in filtered_models {
        println!("  {} ({}) - {} - {} - {}", 
            model.label, 
            model.id, 
            model.tier, 
            model.swe_score, 
            model.ctx
        );
    }
    
    Ok(())
}

async fn handle_test_command(model_id: &str, count: usize, config: &config::Config) -> anyhow::Result<()> {
    println!("Testing model {} with {} requests...", model_id, count);
    
    let models = get_models();
    if let Some(model) = models.iter().find(|m| m.id == model_id) {
        let client = Client::new();
        let api_key = get_api_key(config, &model.provider_key);
        
        for i in 1..=count {
            println!("Test {}/{}...", i, count);
            let result = ping_model(&client, model, api_key.clone()).await;
            
            if let Some(latency) = result.latency {
                println!("  Success: {}ms", latency);
            } else {
                println!("  Failed: {}", result.status);
            }
            
            if i < count {
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    } else {
        println!("Model '{}' not found", model_id);
    }
    
    Ok(())
}
