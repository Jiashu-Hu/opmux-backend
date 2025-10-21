//! Executor service layer - orchestrates LLM execution workflow.

use super::{
    config::ExecutorConfig,
    error::ExecutorError,
    models::{ExecutionParams, ExecutionResult},
    vendors::{openai::OpenAIVendor, traits::LLMVendor},
};
use crate::features::ingress::repository::RoutePlan;
use std::collections::HashMap;
use std::sync::Arc;

/// Service for executing LLM API calls based on routing decisions.
///
/// Manages vendor registration, parameter extraction, and execution orchestration.
pub struct ExecutorService {
    /// Vendor registry: vendor_id → vendor instance
    vendors: HashMap<String, Arc<dyn LLMVendor>>,
    /// Executor configuration (will be used in Task 8.6.4 for retry logic)
    #[allow(dead_code)]
    config: ExecutorConfig,
}

impl ExecutorService {
    /// Creates ExecutorService from configuration.
    ///
    /// Automatically initializes all configured vendors based on the provided configuration.
    ///
    /// # Parameters
    /// - `config` - Executor configuration with vendor settings
    ///
    /// # Returns
    /// ExecutorService instance with initialized vendors
    ///
    /// # Errors
    /// Returns `NoVendorsConfigured` if no vendors are configured
    pub fn from_config(config: ExecutorConfig) -> Result<Self, ExecutorError> {
        let mut vendors: HashMap<String, Arc<dyn LLMVendor>> = HashMap::new();

        // Initialize OpenAI vendor if configured
        if let Some(openai_config) = &config.openai {
            let vendor = OpenAIVendor::new(openai_config.clone());
            vendors.insert("openai".to_string(), Arc::new(vendor));
            tracing::info!("Initialized OpenAI vendor");
        }

        // Future: Initialize Anthropic vendor if configured
        // if let Some(anthropic_key) = &config.anthropic_api_key {
        //     let vendor = AnthropicVendor::new(anthropic_key.clone());
        //     vendors.insert("anthropic".to_string(), Arc::new(vendor));
        //     tracing::info!("Initialized Anthropic vendor");
        // }

        // Validate that at least one vendor is configured
        if vendors.is_empty() {
            return Err(ExecutorError::NoVendorsConfigured);
        }

        Ok(Self { vendors, config })
    }

    /// Returns the number of registered vendors.
    ///
    /// Useful for logging and monitoring vendor availability.
    pub fn vendor_count(&self) -> usize {
        self.vendors.len()
    }

    /// Extracts execution parameters from request payload.
    ///
    /// # Parameters
    /// - `payload` - JSON payload containing execution parameters
    ///
    /// # Returns
    /// Extracted execution parameters
    ///
    /// # Errors
    /// Returns `InvalidPayload` if:
    /// - `messages` field is missing or invalid
    /// - Required fields have wrong types
    #[allow(dead_code)] // Will be used in Task 8.6.6
    fn extract_params(
        payload: &serde_json::Value,
    ) -> Result<ExecutionParams, ExecutorError> {
        // Extract messages (required field)
        let messages = payload
            .get("messages")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .ok_or_else(|| {
                ExecutorError::InvalidPayload(
                    "Missing or invalid 'messages' field".to_string(),
                )
            })?;

        // Extract optional parameters with type validation
        let temperature = payload.get("temperature").and_then(|v| v.as_f64());
        let max_tokens = payload.get("max_tokens").and_then(|v| v.as_i64());
        let top_p = payload.get("top_p").and_then(|v| v.as_f64());
        let stream = payload
            .get("stream")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(ExecutionParams {
            messages,
            temperature,
            max_tokens,
            top_p,
            stream,
        })
    }

    /// Executes LLM call based on routing plan.
    ///
    /// # Parameters
    /// - `plan` - Routing plan from Router Service
    /// - `payload` - Original request payload
    ///
    /// # Returns
    /// Execution result with AI response and metrics
    pub async fn execute(
        &self,
        _plan: &RoutePlan,
        _payload: &serde_json::Value,
    ) -> Result<ExecutionResult, ExecutorError> {
        // Implementation will be added in Task 8.6.6
        todo!("ExecutorService::execute not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_params_with_all_fields() {
        let payload = json!({
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "Hello!"}
            ],
            "temperature": 0.7,
            "max_tokens": 100,
            "top_p": 0.9,
            "stream": true
        });

        let result = ExecutorService::extract_params(&payload);
        assert!(result.is_ok());

        let params = result.unwrap();
        assert_eq!(params.messages.len(), 2);
        assert_eq!(params.messages[0].role, "system");
        assert_eq!(params.messages[0].content, "You are a helpful assistant.");
        assert_eq!(params.messages[1].role, "user");
        assert_eq!(params.messages[1].content, "Hello!");
        assert_eq!(params.temperature, Some(0.7));
        assert_eq!(params.max_tokens, Some(100));
        assert_eq!(params.top_p, Some(0.9));
        assert!(params.stream);
    }

    #[test]
    fn test_extract_params_with_required_only() {
        let payload = json!({
            "messages": [
                {"role": "user", "content": "Hello!"}
            ]
        });

        let result = ExecutorService::extract_params(&payload);
        assert!(result.is_ok());

        let params = result.unwrap();
        assert_eq!(params.messages.len(), 1);
        assert_eq!(params.messages[0].role, "user");
        assert_eq!(params.messages[0].content, "Hello!");
        assert_eq!(params.temperature, None);
        assert_eq!(params.max_tokens, None);
        assert_eq!(params.top_p, None);
        assert!(!params.stream); // Default value
    }

    #[test]
    fn test_extract_params_missing_messages() {
        let payload = json!({
            "temperature": 0.7,
            "max_tokens": 100
        });

        let result = ExecutorService::extract_params(&payload);
        assert!(result.is_err());

        match result {
            Err(ExecutorError::InvalidPayload(msg)) => {
                assert!(msg.contains("messages"));
            }
            _ => panic!("Expected InvalidPayload error"),
        }
    }

    #[test]
    fn test_extract_params_invalid_messages_type() {
        let payload = json!({
            "messages": "not an array"
        });

        let result = ExecutorService::extract_params(&payload);
        assert!(result.is_err());

        match result {
            Err(ExecutorError::InvalidPayload(msg)) => {
                assert!(msg.contains("messages"));
            }
            _ => panic!("Expected InvalidPayload error"),
        }
    }

    #[test]
    fn test_extract_params_invalid_message_structure() {
        let payload = json!({
            "messages": [
                {"role": "user"} // Missing 'content' field
            ]
        });

        let result = ExecutorService::extract_params(&payload);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_params_ignores_invalid_optional_fields() {
        let payload = json!({
            "messages": [
                {"role": "user", "content": "Hello!"}
            ],
            "temperature": "not a number", // Invalid type - should be ignored
            "max_tokens": 100,
            "stream": "yes" // Invalid type - should default to false
        });

        let result = ExecutorService::extract_params(&payload);
        assert!(result.is_ok());

        let params = result.unwrap();
        assert_eq!(params.temperature, None); // Invalid value ignored
        assert_eq!(params.max_tokens, Some(100));
        assert!(!params.stream); // Invalid value defaults to false
    }
}
