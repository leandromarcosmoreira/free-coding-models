use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use home::home_dir;

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
