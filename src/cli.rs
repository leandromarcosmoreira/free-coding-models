use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "free-coding-models")]
#[command(about = "Find the fastest coding LLM models in seconds")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// Show only the best model
    #[arg(short, long)]
    pub best: bool,
    
    /// Show only reliable models (95%+ uptime)
    #[arg(short, long)]
    pub reliable: bool,
    
    /// Filter models suitable for OpenCode
    #[arg(long)]
    pub opencode: bool,
    
    /// Filter models suitable for OpenClaw
    #[arg(long)]
    pub openclaw: bool,
    
    /// Filter by tier (S+, S, A+, A, A-, B+, B, C)
    #[arg(short, long)]
    pub tier: Option<String>,
    
    /// Number of parallel requests
    #[arg(short, long, default_value = "10")]
    pub parallel: usize,
    
    /// Timeout per request in seconds
    #[arg(short, long, default_value = "15")]
    pub timeout: u64,
    
    /// Continuous monitoring mode
    #[arg(short, long)]
    pub watch: bool,
    
    /// JSON output
    #[arg(short, long)]
    pub json: bool,
    
    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configure API keys and settings
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// List all available models
    List {
        /// Filter by provider
        #[arg(short, long)]
        provider: Option<String>,
        
        /// Filter by tier
        #[arg(short, long)]
        tier: Option<String>,
    },
    /// Test a specific model
    Test {
        /// Model ID to test
        model_id: String,
        
        /// Number of test requests
        #[arg(short, long, default_value = "5")]
        count: usize,
    },
    /// Show version and build info
    Version,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Set API key for a provider
    SetKey {
        /// Provider name (nvidia, groq, openai, etc.)
        provider: String,
        
        /// API key value
        key: String,
    },
    /// Remove API key for a provider
    RemoveKey {
        /// Provider name
        provider: String,
    },
    /// List all configured keys
    ListKeys,
    /// Reset all configuration
    Reset,
    /// Show current configuration
    Show,
}

impl Cli {
    pub fn get_config_file() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home.join(".free-coding-models.json")
    }
    
    pub fn should_filter_opencode(&self) -> bool {
        self.opencode || matches!(&self.command, Some(Commands::Test { .. }))
    }
    
    pub fn should_filter_openclaw(&self) -> bool {
        self.openclaw
    }
}
