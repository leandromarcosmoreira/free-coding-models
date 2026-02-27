use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub label: String,
    pub tier: String,
    pub swe_score: String,
    pub ctx: String,
    pub provider_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    pub url: String,
    pub models: Vec<Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResult {
    pub model_id: String,
    pub latency: Option<u128>,
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStats {
    pub model: Model,
    pub avg_latency: f64,
    pub uptime: f64,
    pub success_count: usize,
    pub total_count: usize,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

impl PingResult {
    pub fn success(model_id: String, latency: u128) -> Self {
        Self {
            model_id,
            latency: Some(latency),
            status: "Success".to_string(),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(model_id: String, error: String) -> Self {
        Self {
            model_id,
            latency: None,
            status: error,
            timestamp: chrono::Utc::now(),
        }
    }
}

pub fn get_models() -> Vec<Model> {
    let mut all_models = Vec::new();
    let providers = get_providers();
    for (key, provider) in providers {
        for mut model in provider.models {
            model.provider_key = key.clone();
            all_models.push(model);
        }
    }
    all_models
}

pub fn get_providers() -> HashMap<String, Provider> {
    let mut providers = HashMap::new();

    // NVIDIA NIM models
    providers.insert(
        "nvidia".to_string(),
        Provider {
            name: "NVIDIA NIM".to_string(),
            url: "https://integrate.api.nvidia.com/v1/chat/completions".to_string(),
            models: vec![
                m("deepseek-ai/deepseek-v3.2", "DeepSeek V3.2", "S+", "73.1%", "128k"),
                m("moonshotai/kimi-k2.5", "Kimi K2.5", "S+", "76.8%", "128k"),
                m("z-ai/glm5", "GLM 5", "S+", "77.8%", "128k"),
                m("z-ai/glm4.7", "GLM 4.7", "S+", "73.8%", "200k"),
                m("mistralai/devstral-2-123b-instruct-2512", "Devstral 2 123B", "S+", "72.2%", "256k"),
                m("meta/llama-3.3-70b-instruct", "Llama 3.3 70B", "A+", "68.5%", "128k"),
                m("mistralai/mixtral-8x7b-instruct-v0.1", "Mixtral 8x7B", "A", "62.1%", "32k"),
                m("mistralai/mistral-7b-instruct-v0.2", "Mistral 7B", "B+", "55.3%", "32k"),
            ],
        },
    );

    // Groq models
    providers.insert(
        "groq".to_string(),
        Provider {
            name: "Groq".to_string(),
            url: "https://api.groq.com/openai/v1/chat/completions".to_string(),
            models: vec![
                m("llama-3.3-70b-versatile", "Llama 3.3 70B", "A-", "39.5%", "128k"),
                m("qwen-qwq-32b", "QwQ 32B", "A+", "50.0%", "131k"),
                m("llama-3.1-70b-versatile", "Llama 3.1 70B", "A-", "38.2%", "128k"),
                m("mixtral-8x7b-32768", "Mixtral 8x7B", "A", "35.8%", "32k"),
            ],
        },
    );

    // OpenRouter models
    providers.insert(
        "openrouter".to_string(),
        Provider {
            name: "OpenRouter".to_string(),
            url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
            models: vec![
                m("deepseek/deepseek-chat", "DeepSeek Chat", "S+", "73.1%", "128k"),
                m("qwen/qwen-2.5-coder-32b-instruct", "Qwen 2.5 Coder 32B", "S+", "76.8%", "32k"),
                m("meta-llama/llama-3.3-70b-instruct", "Llama 3.3 70B", "A+", "68.5%", "128k"),
                m("mistralai/mixtral-8x7b-instruct", "Mixtral 8x7B", "A", "62.1%", "32k"),
            ],
        },
    );

    providers
}

fn m(id: &str, label: &str, tier: &str, swe: &str, ctx: &str) -> Model {
    Model {
        id: id.to_string(),
        label: label.to_string(),
        tier: tier.to_string(),
        swe_score: swe.to_string(),
        ctx: ctx.to_string(),
        provider_key: String::new(), // Will be populated by get_models
    }
}

pub fn filter_models_by_tier(models: &[Model], tier_filter: &str) -> Vec<&Model> {
    if tier_filter == "all" {
        return models.iter().collect();
    }
    
    models.iter().filter(|m| {
        match tier_filter {
            "s+" => m.tier == "S+",
            "s" => m.tier == "S+",
            "a+" => m.tier == "S+" || m.tier == "A+",
            "a" => m.tier == "S+" || m.tier == "A+" || m.tier == "A",
            "a-" => m.tier == "S+" || m.tier == "A+" || m.tier == "A" || m.tier == "A-",
            "b+" => m.tier == "S+" || m.tier == "A+" || m.tier == "A" || m.tier == "A-" || m.tier == "B+",
            "b" => m.tier == "S+" || m.tier == "A+" || m.tier == "A" || m.tier == "A-" || m.tier == "B+" || m.tier == "B",
            "c" => true, // All tiers
            _ => false,
        }
    }).collect()
}

pub fn get_best_model(models: &[Model], stats: &[ModelStats]) -> Option<&ModelStats> {
    let available_models: Vec<&ModelStats> = stats.iter()
        .filter(|s| s.uptime >= 95.0 && s.avg_latency > 0.0)
        .collect();
    
    available_models.iter()
        .min_by(|a, b| a.avg_latency.partial_cmp(&b.avg_latency).unwrap())
        .copied()
}
