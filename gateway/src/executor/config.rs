//! Configuration for Executor Layer.

use std::collections::HashMap;
use std::env;

/// Model pricing information.
///
/// Stores the cost per 1000 tokens for prompt and completion.
#[derive(Debug, Clone)]
pub struct ModelPricing {
    /// Price per 1000 prompt tokens in USD
    pub prompt_price_per_1k: f64,
    /// Price per 1000 completion tokens in USD
    pub completion_price_per_1k: f64,
}

impl ModelPricing {
    /// Creates a new model pricing configuration.
    pub fn new(prompt_price_per_1k: f64, completion_price_per_1k: f64) -> Self {
        Self {
            prompt_price_per_1k,
            completion_price_per_1k,
        }
    }
}

/// OpenAI vendor configuration.
#[derive(Debug, Clone)]
pub struct OpenAIConfig {
    /// API key for authentication
    pub api_key: String,
    /// Base URL for API endpoint
    pub base_url: String,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
    /// List of supported model IDs
    pub supported_models: Vec<String>,
    /// Pricing information for each model
    pub pricing: HashMap<String, ModelPricing>,
}

impl OpenAIConfig {
    /// Loads OpenAI configuration from environment variables.
    ///
    /// # Environment Variables
    /// - `OPENAI_API_KEY` - API key (required)
    /// - `OPENAI_BASE_URL` - Base URL (default: https://api.openai.com/v1)
    /// - `OPENAI_TIMEOUT_MS` - Request timeout (default: 30000)
    pub fn from_env() -> Self {
        let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
        let base_url = env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let timeout_ms = env::var("OPENAI_TIMEOUT_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30000);

        // Default pricing (as of 2024, subject to change)
        let mut pricing = HashMap::new();
        pricing.insert("gpt-4".to_string(), ModelPricing::new(0.03, 0.06));
        pricing.insert("gpt-4-turbo".to_string(), ModelPricing::new(0.01, 0.03));
        pricing.insert(
            "gpt-3.5-turbo".to_string(),
            ModelPricing::new(0.0005, 0.0015),
        );

        Self {
            api_key,
            base_url,
            timeout_ms,
            supported_models: vec![
                "gpt-4".to_string(),
                "gpt-4-turbo".to_string(),
                "gpt-3.5-turbo".to_string(),
            ],
            pricing,
        }
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), String> {
        if self.api_key.is_empty() {
            return Err("OPENAI_API_KEY is not set or empty".to_string());
        }
        if self.supported_models.is_empty() {
            return Err("No supported models configured".to_string());
        }
        Ok(())
    }
}

/// Configuration for Executor Layer.
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// OpenAI vendor configuration
    pub openai: Option<OpenAIConfig>,
    /// Anthropic API key (future)
    pub anthropic_api_key: Option<String>,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
    /// Maximum number of retries
    pub max_retries: u32,
}

impl ExecutorConfig {
    /// Loads configuration from environment variables.
    pub fn from_env() -> Self {
        // Load OpenAI configuration if API key is set
        let openai = if env::var("OPENAI_API_KEY").is_ok() {
            Some(OpenAIConfig::from_env())
        } else {
            None
        };

        let anthropic_api_key = env::var("ANTHROPIC_API_KEY").ok();

        let timeout_ms = env::var("EXECUTOR_TIMEOUT_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30000); // Default: 30 seconds

        let max_retries = env::var("EXECUTOR_MAX_RETRIES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3); // Default: 3 retries

        Self {
            openai,
            anthropic_api_key,
            timeout_ms,
            max_retries,
        }
    }

    /// Validates configuration.
    pub fn validate(&self) {
        if self.openai.is_none() && self.anthropic_api_key.is_none() {
            tracing::warn!(
                "No LLM vendor API keys configured. Set OPENAI_API_KEY or ANTHROPIC_API_KEY."
            );
        }

        if let Some(ref config) = self.openai {
            if let Err(e) = config.validate() {
                tracing::warn!("OpenAI configuration invalid: {}", e);
            } else {
                tracing::info!(
                    "OpenAI vendor configured with {} models",
                    config.supported_models.len()
                );
            }
        }

        if let Some(ref key) = self.anthropic_api_key {
            if key.is_empty() {
                tracing::warn!("ANTHROPIC_API_KEY is empty");
            } else {
                tracing::info!("Anthropic vendor configured");
            }
        }

        tracing::info!("Executor timeout: {}ms", self.timeout_ms);
        tracing::info!("Executor max retries: {}", self.max_retries);
    }
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            openai: None,
            anthropic_api_key: None,
            timeout_ms: 30000,
            max_retries: 3,
        }
    }
}
