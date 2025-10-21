//! OpenAI vendor implementation.
//!
//! Integrates with OpenAI Chat Completions API for LLM execution.

use crate::executor::{
    config::OpenAIConfig,
    error::ExecutorError,
    models::{ExecutionParams, ExecutionResult, Message},
    vendors::traits::LLMVendor,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// OpenAI Chat Completions API request.
#[derive(Debug, Deserialize, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,
}

/// OpenAI Chat Completions API response.
#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    #[allow(dead_code)]
    id: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: i64,
    completion_tokens: i64,
    #[allow(dead_code)]
    total_tokens: i64,
}

/// OpenAI vendor implementation.
pub struct OpenAIVendor {
    config: OpenAIConfig,
    client: Client,
}

impl OpenAIVendor {
    /// Creates a new OpenAI vendor instance from configuration.
    ///
    /// # Parameters
    /// - `config` - OpenAI configuration with API key, base URL, and pricing
    pub fn new(config: OpenAIConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LLMVendor for OpenAIVendor {
    fn vendor_id(&self) -> &str {
        "openai"
    }

    fn supports_model(&self, model: &str) -> bool {
        // Business logic: check if model is in the supported list
        self.config.supported_models.contains(&model.to_string())
    }

    fn calculate_cost(
        &self,
        prompt_tokens: i64,
        completion_tokens: i64,
        model: &str,
    ) -> f64 {
        // Business logic: calculate cost based on pricing configuration
        if let Some(pricing) = self.config.pricing.get(model) {
            (prompt_tokens as f64 * pricing.prompt_price_per_1k
                + completion_tokens as f64 * pricing.completion_price_per_1k)
                / 1000.0
        } else {
            // Fallback to 0.0 if pricing not found
            tracing::warn!("No pricing found for model: {}", model);
            0.0
        }
    }

    async fn execute(
        &self,
        model: &str,
        params: ExecutionParams,
    ) -> Result<ExecutionResult, ExecutorError> {
        if !self.supports_model(model) {
            return Err(ExecutorError::UnsupportedModel(
                model.to_string(),
                self.vendor_id().to_string(),
            ));
        }

        let request = ChatCompletionRequest {
            messages: params.messages,
            temperature: params.temperature,
            max_tokens: params.max_tokens,
            top_p: params.top_p,
            model: model.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ExecutorError::ApiCallFailed(format!(
                "OpenAI API error {}: {}",
                status, error_text
            )));
        }

        let api_response: ChatCompletionResponse = response.json().await?;

        let choice = api_response.choices.first().ok_or_else(|| {
            ExecutorError::ApiCallFailed("No choices in response".to_string())
        })?;

        let content = choice.message.content.clone();
        let finish_reason = choice.finish_reason.clone();

        let total_cost = self.calculate_cost(
            api_response.usage.prompt_tokens,
            api_response.usage.completion_tokens,
            model,
        );

        Ok(ExecutionResult {
            content,
            model_used: model.to_string(),
            prompt_tokens: api_response.usage.prompt_tokens,
            completion_tokens: api_response.usage.completion_tokens,
            total_cost,
            finish_reason,
        })
    }
}
