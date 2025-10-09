//! Configuration for Executor Layer.

use std::env;

/// Configuration for Executor Layer.
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// OpenAI API key
    pub openai_api_key: Option<String>,
    /// Anthropic API key
    pub anthropic_api_key: Option<String>,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
    /// Maximum number of retries
    pub max_retries: u32,
}

impl ExecutorConfig {
    /// Loads configuration from environment variables.
    pub fn from_env() -> Self {
        let openai_api_key = env::var("OPENAI_API_KEY").ok();
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
            openai_api_key,
            anthropic_api_key,
            timeout_ms,
            max_retries,
        }
    }

    /// Validates configuration.
    pub fn validate(&self) {
        if self.openai_api_key.is_none() && self.anthropic_api_key.is_none() {
            tracing::warn!(
                "No LLM vendor API keys configured. Set OPENAI_API_KEY or ANTHROPIC_API_KEY."
            );
        }

        if let Some(ref key) = self.openai_api_key {
            if key.is_empty() {
                tracing::warn!("OPENAI_API_KEY is empty");
            } else {
                tracing::info!("OpenAI vendor configured");
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
            openai_api_key: None,
            anthropic_api_key: None,
            timeout_ms: 30000,
            max_retries: 3,
        }
    }
}

