use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use home::home_dir;
use chrono::{Utc, Duration};
use rand::Rng;
use std::io;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryConfig {
    pub enabled: Option<bool>,
    pub consent_version: i32,
    pub anonymous_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "apiKeys")]
    pub api_keys: HashMap<String, String>,
    pub telemetry: TelemetryConfig,
    pub favorites: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_keys: HashMap::new(),
            telemetry: TelemetryConfig::default(),
            favorites: Vec::new(),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".free-coding-models.json");
    path
}

pub fn load_config() -> Config {
    let path = get_config_path();
    if !path.exists() {
        return Config::default();
    }

    let content = fs::read_to_string(path).expect("Could not read config file");
    serde_json::from_str(&content).unwrap_or_else(|_| Config::default())
}

pub fn save_config(config: &Config) {
    let path = get_config_path();
    let content = serde_json::to_string_pretty(config).expect("Could not serialize config");
    fs::write(path, content).expect("Could not write config file");
}

pub fn get_api_key(config: &Config, provider: &str) -> Option<String> {
    config.api_keys.get(provider).cloned()
}

pub fn ensure_favorites_config(config: &mut Config, model_id: &str) {
    if !config.favorites.contains(&model_id.to_string()) {
        config.favorites.push(model_id.to_string());
        save_config(config);
    }
}

pub fn toggle_favorite_model(config: &mut Config, model_id: &str) {
    if let Some(pos) = config.favorites.iter().position(|id| id == model_id) {
        config.favorites.remove(pos);
    } else {
        config.favorites.push(model_id.to_string());
    }
    save_config(config);
}

pub fn prompt_telemetry_consent() -> bool {
    // In Rust CLI, we'd need to implement a proper prompt
    // For now, return true for consent
    true
}

pub fn get_telemetry_distinct_id() -> String {
    let config = load_config();
    if let Some(id) = config.telemetry.anonymous_id {
        id
    } else {
        let mut rng = rand::thread_rng();
        let id: String = std::iter::repeat(()).map(|_|
            rng.sample(rand::distributions::Alphanumeric) as char
        ).take(32).collect();
        id
    }
}

pub fn send_usage_telemetry() {
    // Fire-and-forget telemetry
    tokio::spawn(async move {
        // Telemetry logic here
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.api_keys.is_empty());
        assert!(config.favorites.is_empty());
    }

    #[test]
    fn test_config_path() {
        let path = get_config_path();
        assert!(path.to_str().unwrap().ends_with(".free-coding-models.json"));
    }
}
