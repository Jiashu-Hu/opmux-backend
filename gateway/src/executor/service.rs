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
    /// Executor configuration for retry logic and timeout settings
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

    /// Selects vendor by vendor_id.
    ///
    /// # Parameters
    /// - `vendor_id` - Vendor identifier (e.g., "openai", "anthropic")
    ///
    /// # Returns
    /// Arc reference to the vendor implementation
    ///
    /// # Errors
    /// Returns `UnsupportedVendor` if vendor_id is not found in registry
    fn get_vendor(&self, vendor_id: &str) -> Result<Arc<dyn LLMVendor>, ExecutorError> {
        self.vendors
            .get(vendor_id)
            .cloned()
            .ok_or_else(|| ExecutorError::UnsupportedVendor(vendor_id.to_string()))
    }

    /// Executes LLM call with retry logic and exponential backoff.
    ///
    /// # Flow
    /// 1. Get vendor by vendor_id
    /// 2. Validate model support
    /// 3. Attempt execution with retry loop
    /// 4. Apply exponential backoff between retries (1s, 2s, 4s, 8s...)
    ///
    /// # Parameters
    /// - `vendor_id` - Vendor identifier
    /// - `model_id` - Model identifier
    /// - `params` - Execution parameters
    ///
    /// # Returns
    /// Execution result with AI response and metrics
    ///
    /// # Errors
    /// Returns error if:
    /// - Vendor not found
    /// - Model not supported
    /// - All retry attempts exhausted
    /// - Non-retryable error occurs
    async fn execute_with_retry(
        &self,
        vendor_id: &str,
        model_id: &str,
        params: &ExecutionParams,
    ) -> Result<ExecutionResult, ExecutorError> {
        // Get vendor and validate model support
        let vendor = self.get_vendor(vendor_id)?;

        if !vendor.supports_model(model_id) {
            return Err(ExecutorError::UnsupportedModel(
                vendor_id.to_string(),
                model_id.to_string(),
            ));
        }

        let max_retries = self.config.max_retries;
        let mut last_error = None;

        // Retry loop with exponential backoff
        for attempt in 0..=max_retries {
            if attempt > 0 {
                // Exponential backoff: 1s, 2s, 4s, 8s, ...
                let backoff_ms = 1000 * (2_u64.pow(attempt - 1));
                tracing::info!(
                    "Retrying execution: attempt {}/{}, vendor={}, model={}, backoff={}ms",
                    attempt,
                    max_retries,
                    vendor_id,
                    model_id,
                    backoff_ms
                );
                tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms)).await;
            }

            // Attempt execution
            match vendor.execute(model_id, params.clone()).await {
                Ok(result) => {
                    if attempt > 0 {
                        tracing::info!(
                            "Execution succeeded after {} retries: vendor={}, model={}",
                            attempt,
                            vendor_id,
                            model_id
                        );
                    }
                    return Ok(result);
                }
                Err(e) => {
                    // Determine if error is retryable
                    if Self::is_retryable_error(&e) {
                        tracing::warn!(
                            "Retryable error on attempt {}/{}: vendor={}, model={}, error={:?}",
                            attempt,
                            max_retries,
                            vendor_id,
                            model_id,
                            e
                        );
                        last_error = Some(e);
                        continue;
                    } else {
                        // Non-retryable error, fail immediately
                        tracing::error!(
                            "Non-retryable error: vendor={}, model={}, error={:?}",
                            vendor_id,
                            model_id,
                            e
                        );
                        return Err(e);
                    }
                }
            }
        }

        // All retries exhausted
        tracing::error!(
            "Max retries exceeded: vendor={}, model={}, attempts={}",
            vendor_id,
            model_id,
            max_retries + 1
        );
        Err(last_error.unwrap_or_else(|| {
            ExecutorError::ApiCallFailed("Max retries exceeded".to_string())
        }))
    }

    /// Determines if an error is retryable.
    ///
    /// # Retryable Errors
    /// - NetworkError - Network connectivity issues
    /// - TimeoutError - Request timeout
    /// - RateLimitExceeded - Vendor rate limit hit
    /// - ApiCallFailed - Generic API call failure
    ///
    /// # Non-Retryable Errors
    /// - AuthenticationFailed - Invalid API key (won't fix with retry)
    /// - InvalidPayload - Bad request format (won't fix with retry)
    /// - UnsupportedVendor - Vendor not configured (won't fix with retry)
    /// - UnsupportedModel - Model not supported (won't fix with retry)
    fn is_retryable_error(error: &ExecutorError) -> bool {
        matches!(
            error,
            ExecutorError::NetworkError(_)
                | ExecutorError::TimeoutError(_)
                | ExecutorError::RateLimitExceeded(_)
                | ExecutorError::ApiCallFailed(_)
        )
    }

    /// Executes fallback plans sequentially.
    ///
    /// # Flow
    /// 1. Check if fallback plans exist
    /// 2. Try each fallback plan sequentially
    /// 3. Each fallback gets full retry logic via execute_with_retry()
    /// 4. Return on first successful fallback
    /// 5. Return primary error if all fallbacks fail
    ///
    /// # Parameters
    /// - `fallback_plans` - List of fallback routing plans
    /// - `params` - Execution parameters (shared across all attempts)
    /// - `primary_error` - Error from primary execution attempt
    ///
    /// # Returns
    /// Execution result from first successful fallback
    ///
    /// # Errors
    /// Returns primary error if no fallbacks exist or all fallbacks fail
    async fn execute_fallbacks(
        &self,
        fallback_plans: &[RoutePlan],
        params: &ExecutionParams,
        primary_error: ExecutorError,
    ) -> Result<ExecutionResult, ExecutorError> {
        // No fallbacks available, return primary error
        if fallback_plans.is_empty() {
            tracing::warn!("No fallback plans available, returning primary error");
            return Err(primary_error);
        }

        tracing::info!(
            "Primary execution failed, attempting {} fallback plans",
            fallback_plans.len()
        );

        // Try each fallback sequentially
        for (index, fallback) in fallback_plans.iter().enumerate() {
            tracing::info!(
                "Trying fallback {}/{}: vendor={}, model={}",
                index + 1,
                fallback_plans.len(),
                fallback.vendor_id,
                fallback.model_id
            );

            match self
                .execute_with_retry(&fallback.vendor_id, &fallback.model_id, params)
                .await
            {
                Ok(result) => {
                    tracing::info!(
                        "Fallback {} succeeded: vendor={}, model={}",
                        index + 1,
                        fallback.vendor_id,
                        fallback.model_id
                    );
                    return Ok(result);
                }
                Err(e) => {
                    tracing::warn!(
                        "Fallback {} failed: vendor={}, model={}, error={:?}",
                        index + 1,
                        fallback.vendor_id,
                        fallback.model_id,
                        e
                    );
                    // Continue to next fallback
                    continue;
                }
            }
        }

        // All fallbacks exhausted, return primary error
        tracing::error!(
            "All {} fallback plans failed, returning primary error",
            fallback_plans.len()
        );
        Err(primary_error)
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
    /// # Flow
    /// 1. Extract parameters from payload
    /// 2. Try primary plan with retry logic
    /// 3. On failure, try fallback plans sequentially
    /// 4. Return result or error
    ///
    /// # Parameters
    /// - `plan` - Routing plan from Router Service
    /// - `payload` - Original request payload
    ///
    /// # Returns
    /// Execution result with AI response and metrics
    ///
    /// # Errors
    /// Returns error if:
    /// - Payload is invalid
    /// - Primary execution fails and no fallbacks succeed
    pub async fn execute(
        &self,
        plan: &RoutePlan,
        payload: &serde_json::Value,
    ) -> Result<ExecutionResult, ExecutorError> {
        // Extract parameters once (shared across retries and fallbacks)
        let params = Self::extract_params(payload)?;

        tracing::info!(
            "Executing LLM call: vendor={}, model={}",
            plan.vendor_id,
            plan.model_id
        );

        // Try primary plan with retry logic
        match self
            .execute_with_retry(&plan.vendor_id, &plan.model_id, &params)
            .await
        {
            Ok(result) => {
                tracing::info!(
                    "Primary execution succeeded: vendor={}, model={}, tokens={}, cost=${}",
                    plan.vendor_id,
                    plan.model_id,
                    result.prompt_tokens + result.completion_tokens,
                    result.total_cost
                );
                Ok(result)
            }
            Err(primary_error) => {
                tracing::warn!(
                    "Primary execution failed: vendor={}, model={}, error={:?}",
                    plan.vendor_id,
                    plan.model_id,
                    primary_error
                );

                // Try fallback plans
                self.execute_fallbacks(&plan.fallback_plans, &params, primary_error)
                    .await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::config::OpenAIConfig;
    use serde_json::json;

    // Helper function to create a test ExecutorService with OpenAI vendor
    fn create_test_executor_service() -> ExecutorService {
        let config = ExecutorConfig {
            openai: Some(OpenAIConfig::from_env()),
            anthropic_api_key: None, // Not used in tests
            max_retries: 3,
            timeout_ms: 30000,
        };
        ExecutorService::from_config(config).expect("Failed to create test executor")
    }

    #[test]
    fn test_get_vendor_success() {
        let service = create_test_executor_service();

        let result = service.get_vendor("openai");
        assert!(result.is_ok());

        let vendor = result.unwrap();
        assert_eq!(vendor.vendor_id(), "openai");
    }

    #[test]
    fn test_get_vendor_not_found() {
        let service = create_test_executor_service();

        let result = service.get_vendor("unknown_vendor");
        assert!(result.is_err());

        match result {
            Err(ExecutorError::UnsupportedVendor(vendor_id)) => {
                assert_eq!(vendor_id, "unknown_vendor");
            }
            _ => panic!("Expected UnsupportedVendor error"),
        }
    }

    #[test]
    fn test_get_vendor_case_sensitive() {
        let service = create_test_executor_service();

        // Vendor IDs are case-sensitive
        let result = service.get_vendor("OpenAI"); // Wrong case
        assert!(result.is_err());

        match result {
            Err(ExecutorError::UnsupportedVendor(vendor_id)) => {
                assert_eq!(vendor_id, "OpenAI");
            }
            _ => panic!("Expected UnsupportedVendor error"),
        }
    }

    #[test]
    fn test_vendor_count() {
        let service = create_test_executor_service();

        // Should have 1 vendor (OpenAI) if OPENAI_API_KEY is set
        // Otherwise, from_config would have failed
        assert_eq!(service.vendor_count(), 1);
    }

    #[test]
    fn test_is_retryable_error_network_error() {
        let error = ExecutorError::NetworkError("Connection failed".to_string());
        assert!(ExecutorService::is_retryable_error(&error));
    }

    #[test]
    fn test_is_retryable_error_timeout() {
        let error = ExecutorError::TimeoutError(30000);
        assert!(ExecutorService::is_retryable_error(&error));
    }

    #[test]
    fn test_is_retryable_error_rate_limit() {
        let error = ExecutorError::RateLimitExceeded("openai".to_string());
        assert!(ExecutorService::is_retryable_error(&error));
    }

    #[test]
    fn test_is_retryable_error_api_call_failed() {
        let error = ExecutorError::ApiCallFailed("500 Internal Server Error".to_string());
        assert!(ExecutorService::is_retryable_error(&error));
    }

    #[test]
    fn test_is_not_retryable_error_authentication() {
        let error = ExecutorError::AuthenticationFailed("openai".to_string());
        assert!(!ExecutorService::is_retryable_error(&error));
    }

    #[test]
    fn test_is_not_retryable_error_invalid_payload() {
        let error = ExecutorError::InvalidPayload("Missing messages".to_string());
        assert!(!ExecutorService::is_retryable_error(&error));
    }

    #[test]
    fn test_is_not_retryable_error_unsupported_vendor() {
        let error = ExecutorError::UnsupportedVendor("unknown".to_string());
        assert!(!ExecutorService::is_retryable_error(&error));
    }

    #[test]
    fn test_is_not_retryable_error_unsupported_model() {
        let error =
            ExecutorError::UnsupportedModel("openai".to_string(), "gpt-5".to_string());
        assert!(!ExecutorService::is_retryable_error(&error));
    }

    #[tokio::test]
    async fn test_execute_fallbacks_empty_list() {
        let service = create_test_executor_service();
        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };
        let primary_error = ExecutorError::ApiCallFailed("Primary failed".to_string());

        let result = service
            .execute_fallbacks(&[], &params, primary_error.clone())
            .await;

        assert!(result.is_err());
        // Should return the primary error when no fallbacks exist
        match result {
            Err(ExecutorError::ApiCallFailed(msg)) => {
                assert_eq!(msg, "Primary failed");
            }
            _ => panic!("Expected ApiCallFailed error"),
        }
    }

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
