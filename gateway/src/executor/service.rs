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
    use crate::executor::vendors::LLMVendor;
    use async_trait::async_trait;
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    /// Configurable mock vendor for testing.
    ///
    /// Supports various failure scenarios:
    /// - Fail N times then succeed
    /// - Always fail with specific error
    /// - Always succeed
    /// - Model support configuration
    #[derive(Clone)]
    struct MockVendor {
        vendor_id: String,
        supported_models: Vec<String>,
        /// Number of times to fail before succeeding (None = always succeed)
        fail_count: Option<Arc<AtomicUsize>>,
        /// Error to return on failure
        failure_error: Option<ExecutorError>,
    }

    impl MockVendor {
        /// Creates a mock vendor that always succeeds.
        fn new_success(vendor_id: &str, models: Vec<&str>) -> Self {
            Self {
                vendor_id: vendor_id.to_string(),
                supported_models: models.iter().map(|s| s.to_string()).collect(),
                fail_count: None,
                failure_error: None,
            }
        }

        /// Creates a mock vendor that fails N times then succeeds.
        fn new_fail_then_succeed(
            vendor_id: &str,
            models: Vec<&str>,
            fail_times: usize,
            error: ExecutorError,
        ) -> Self {
            Self {
                vendor_id: vendor_id.to_string(),
                supported_models: models.iter().map(|s| s.to_string()).collect(),
                fail_count: Some(Arc::new(AtomicUsize::new(fail_times))),
                failure_error: Some(error),
            }
        }

        /// Creates a mock vendor that always fails.
        fn new_always_fail(
            vendor_id: &str,
            models: Vec<&str>,
            error: ExecutorError,
        ) -> Self {
            Self {
                vendor_id: vendor_id.to_string(),
                supported_models: models.iter().map(|s| s.to_string()).collect(),
                fail_count: Some(Arc::new(AtomicUsize::new(usize::MAX))),
                failure_error: Some(error),
            }
        }
    }

    #[async_trait]
    impl LLMVendor for MockVendor {
        async fn execute(
            &self,
            model: &str,
            _params: ExecutionParams,
        ) -> Result<ExecutionResult, ExecutorError> {
            // Check if we should fail
            if let Some(ref counter) = self.fail_count {
                let remaining = counter.load(Ordering::SeqCst);
                if remaining > 0 {
                    counter.fetch_sub(1, Ordering::SeqCst);
                    return Err(self.failure_error.clone().unwrap());
                }
            }

            // Success case
            Ok(ExecutionResult {
                content: format!("Mock response from {} using {}", self.vendor_id, model),
                model_used: model.to_string(),
                prompt_tokens: 10,
                completion_tokens: 20,
                total_cost: 0.001,
                finish_reason: "stop".to_string(),
            })
        }

        fn vendor_id(&self) -> &str {
            &self.vendor_id
        }

        fn supports_model(&self, model: &str) -> bool {
            self.supported_models.contains(&model.to_string())
        }

        fn calculate_cost(
            &self,
            _prompt_tokens: i64,
            _completion_tokens: i64,
            _model: &str,
        ) -> f64 {
            0.001
        }
    }

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

    // Helper function to create ExecutorService with mock vendors
    fn create_mock_executor_service(
        vendors: Vec<(String, Arc<dyn LLMVendor>)>,
    ) -> ExecutorService {
        let mut vendor_map = HashMap::new();
        for (id, vendor) in vendors {
            vendor_map.insert(id, vendor);
        }

        let config = ExecutorConfig {
            openai: None,
            anthropic_api_key: None,
            max_retries: 3,
            timeout_ms: 30000,
        };

        ExecutorService {
            vendors: vendor_map,
            config,
        }
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

    // ========================================
    // Comprehensive Tests for execute_with_retry()
    // ========================================

    #[tokio::test]
    async fn test_execute_with_retry_success_first_attempt() {
        let mock_vendor = MockVendor::new_success("mock", vec!["model-1"]);
        let service = create_mock_executor_service(vec![(
            "mock".to_string(),
            Arc::new(mock_vendor),
        )]);

        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };

        let result = service.execute_with_retry("mock", "model-1", &params).await;

        assert!(result.is_ok());
        let exec_result = result.unwrap();
        assert_eq!(exec_result.model_used, "model-1");
        assert!(exec_result.content.contains("Mock response"));
    }

    #[tokio::test(start_paused = true)]
    async fn test_execute_with_retry_fail_once_then_succeed() {
        // Vendor fails once with retryable error, then succeeds
        let mock_vendor = MockVendor::new_fail_then_succeed(
            "mock",
            vec!["model-1"],
            1,
            ExecutorError::NetworkError("Connection timeout".to_string()),
        );
        let service = create_mock_executor_service(vec![(
            "mock".to_string(),
            Arc::new(mock_vendor),
        )]);

        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };

        // Spawn the test in a background task
        let handle = tokio::spawn(async move {
            service.execute_with_retry("mock", "model-1", &params).await
        });

        // Fast-forward time to complete the 1-second backoff instantly
        tokio::time::advance(std::time::Duration::from_secs(1)).await;

        // Wait for the background task to complete
        let result = handle.await.unwrap();

        assert!(result.is_ok());
        let exec_result = result.unwrap();
        assert_eq!(exec_result.model_used, "model-1");
    }

    #[tokio::test(start_paused = true)]
    async fn test_execute_with_retry_fail_twice_then_succeed() {
        // Vendor fails twice with retryable error, then succeeds
        let mock_vendor = MockVendor::new_fail_then_succeed(
            "mock",
            vec!["model-1"],
            2,
            ExecutorError::TimeoutError(30000),
        );
        let service = create_mock_executor_service(vec![(
            "mock".to_string(),
            Arc::new(mock_vendor),
        )]);

        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };

        // Spawn the test in a background task
        let handle = tokio::spawn(async move {
            service.execute_with_retry("mock", "model-1", &params).await
        });

        // Fast-forward time to complete backoff delays (1s + 2s = 3s total)
        tokio::time::advance(std::time::Duration::from_secs(3)).await;

        // Wait for the background task to complete
        let result = handle.await.unwrap();

        assert!(result.is_ok());
        let exec_result = result.unwrap();
        assert_eq!(exec_result.model_used, "model-1");
    }

    #[tokio::test(start_paused = true)]
    async fn test_execute_with_retry_exhaust_retries() {
        // Vendor always fails with retryable error (max_retries = 3)
        let mock_vendor = MockVendor::new_always_fail(
            "mock",
            vec!["model-1"],
            ExecutorError::RateLimitExceeded("mock".to_string()),
        );
        let service = create_mock_executor_service(vec![(
            "mock".to_string(),
            Arc::new(mock_vendor),
        )]);

        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };

        // Spawn the test in a background task
        let handle = tokio::spawn(async move {
            service.execute_with_retry("mock", "model-1", &params).await
        });

        // Fast-forward time to complete all backoff delays (1s + 2s + 4s = 7s total)
        tokio::time::advance(std::time::Duration::from_secs(7)).await;

        // Wait for the background task to complete
        let result = handle.await.unwrap();

        assert!(result.is_err());
        match result {
            Err(ExecutorError::RateLimitExceeded(_)) => {}
            _ => panic!("Expected RateLimitExceeded error"),
        }
    }

    #[tokio::test]
    async fn test_execute_with_retry_non_retryable_error() {
        // Vendor fails with non-retryable error (should fail immediately)
        let mock_vendor = MockVendor::new_always_fail(
            "mock",
            vec!["model-1"],
            ExecutorError::AuthenticationFailed("mock".to_string()),
        );
        let service = create_mock_executor_service(vec![(
            "mock".to_string(),
            Arc::new(mock_vendor),
        )]);

        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };

        let result = service.execute_with_retry("mock", "model-1", &params).await;

        assert!(result.is_err());
        match result {
            Err(ExecutorError::AuthenticationFailed(_)) => {}
            _ => panic!("Expected AuthenticationFailed error"),
        }
    }

    #[tokio::test]
    async fn test_execute_with_retry_unsupported_model() {
        let mock_vendor = MockVendor::new_success("mock", vec!["model-1"]);
        let service = create_mock_executor_service(vec![(
            "mock".to_string(),
            Arc::new(mock_vendor),
        )]);

        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };

        // Try to execute with unsupported model
        let result = service
            .execute_with_retry("mock", "unsupported-model", &params)
            .await;

        assert!(result.is_err());
        match result {
            Err(ExecutorError::UnsupportedModel(vendor, model)) => {
                assert_eq!(vendor, "mock");
                assert_eq!(model, "unsupported-model");
            }
            _ => panic!("Expected UnsupportedModel error"),
        }
    }

    #[tokio::test]
    async fn test_execute_with_retry_vendor_not_found() {
        let mock_vendor = MockVendor::new_success("mock", vec!["model-1"]);
        let service = create_mock_executor_service(vec![(
            "mock".to_string(),
            Arc::new(mock_vendor),
        )]);

        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };

        // Try to execute with unknown vendor
        let result = service
            .execute_with_retry("unknown", "model-1", &params)
            .await;

        assert!(result.is_err());
        match result {
            Err(ExecutorError::UnsupportedVendor(vendor)) => {
                assert_eq!(vendor, "unknown");
            }
            _ => panic!("Expected UnsupportedVendor error"),
        }
    }

    // ========================================
    // Comprehensive Tests for execute_fallbacks()
    // ========================================

    #[tokio::test]
    async fn test_execute_fallbacks_first_fallback_succeeds() {
        let fallback1 = MockVendor::new_success("fallback1", vec!["model-1"]);
        let fallback2 = MockVendor::new_success("fallback2", vec!["model-2"]);

        let service = create_mock_executor_service(vec![
            ("fallback1".to_string(), Arc::new(fallback1)),
            ("fallback2".to_string(), Arc::new(fallback2)),
        ]);

        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };

        let fallback_plans = vec![
            RoutePlan {
                vendor_id: "fallback1".to_string(),
                model_id: "model-1".to_string(),
                fallback_plans: vec![],
            },
            RoutePlan {
                vendor_id: "fallback2".to_string(),
                model_id: "model-2".to_string(),
                fallback_plans: vec![],
            },
        ];

        let primary_error = ExecutorError::ApiCallFailed("Primary failed".to_string());

        let result = service
            .execute_fallbacks(&fallback_plans, &params, primary_error)
            .await;

        assert!(result.is_ok());
        let exec_result = result.unwrap();
        assert_eq!(exec_result.model_used, "model-1"); // First fallback succeeded
    }

    #[tokio::test(start_paused = true)]
    async fn test_execute_fallbacks_second_fallback_succeeds() {
        // First fallback always fails, second succeeds
        let fallback1 = MockVendor::new_always_fail(
            "fallback1",
            vec!["model-1"],
            ExecutorError::NetworkError("Fallback1 failed".to_string()),
        );
        let fallback2 = MockVendor::new_success("fallback2", vec!["model-2"]);

        let service = create_mock_executor_service(vec![
            ("fallback1".to_string(), Arc::new(fallback1)),
            ("fallback2".to_string(), Arc::new(fallback2)),
        ]);

        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };

        let fallback_plans = vec![
            RoutePlan {
                vendor_id: "fallback1".to_string(),
                model_id: "model-1".to_string(),
                fallback_plans: vec![],
            },
            RoutePlan {
                vendor_id: "fallback2".to_string(),
                model_id: "model-2".to_string(),
                fallback_plans: vec![],
            },
        ];

        let primary_error = ExecutorError::ApiCallFailed("Primary failed".to_string());

        // Spawn the test in a background task
        let handle = tokio::spawn(async move {
            service
                .execute_fallbacks(&fallback_plans, &params, primary_error)
                .await
        });

        // Fast-forward time to complete fallback1's retry attempts (1s + 2s + 4s = 7s)
        tokio::time::advance(std::time::Duration::from_secs(7)).await;

        // Wait for the background task to complete
        let result = handle.await.unwrap();

        assert!(result.is_ok());
        let exec_result = result.unwrap();
        assert_eq!(exec_result.model_used, "model-2"); // Second fallback succeeded
    }

    #[tokio::test(start_paused = true)]
    async fn test_execute_fallbacks_all_fail_returns_primary_error() {
        // All fallbacks fail
        let fallback1 = MockVendor::new_always_fail(
            "fallback1",
            vec!["model-1"],
            ExecutorError::NetworkError("Fallback1 failed".to_string()),
        );
        let fallback2 = MockVendor::new_always_fail(
            "fallback2",
            vec!["model-2"],
            ExecutorError::TimeoutError(30000),
        );

        let service = create_mock_executor_service(vec![
            ("fallback1".to_string(), Arc::new(fallback1)),
            ("fallback2".to_string(), Arc::new(fallback2)),
        ]);

        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };

        let fallback_plans = vec![
            RoutePlan {
                vendor_id: "fallback1".to_string(),
                model_id: "model-1".to_string(),
                fallback_plans: vec![],
            },
            RoutePlan {
                vendor_id: "fallback2".to_string(),
                model_id: "model-2".to_string(),
                fallback_plans: vec![],
            },
        ];

        let primary_error = ExecutorError::ApiCallFailed("Primary failed".to_string());

        // Spawn the test in a background task
        let handle = tokio::spawn(async move {
            service
                .execute_fallbacks(&fallback_plans, &params, primary_error.clone())
                .await
        });

        // Fast-forward time to complete both fallbacks' retry attempts (7s + 7s = 14s)
        tokio::time::advance(std::time::Duration::from_secs(14)).await;

        // Wait for the background task to complete
        let result = handle.await.unwrap();

        assert!(result.is_err());
        // Should return primary error, not fallback errors
        match result {
            Err(ExecutorError::ApiCallFailed(msg)) => {
                assert_eq!(msg, "Primary failed");
            }
            _ => panic!("Expected primary error (ApiCallFailed)"),
        }
    }

    #[tokio::test(start_paused = true)]
    async fn test_execute_fallbacks_with_retry_logic() {
        // Fallback fails twice then succeeds (tests that fallbacks get retry logic)
        let fallback1 = MockVendor::new_fail_then_succeed(
            "fallback1",
            vec!["model-1"],
            2,
            ExecutorError::RateLimitExceeded("fallback1".to_string()),
        );

        let service = create_mock_executor_service(vec![(
            "fallback1".to_string(),
            Arc::new(fallback1),
        )]);

        let params = ExecutionParams {
            messages: vec![],
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
        };

        let fallback_plans = vec![RoutePlan {
            vendor_id: "fallback1".to_string(),
            model_id: "model-1".to_string(),
            fallback_plans: vec![],
        }];

        let primary_error = ExecutorError::ApiCallFailed("Primary failed".to_string());

        // Spawn the test in a background task
        let handle = tokio::spawn(async move {
            service
                .execute_fallbacks(&fallback_plans, &params, primary_error)
                .await
        });

        // Fast-forward time to complete backoff delays (1s + 2s = 3s total)
        tokio::time::advance(std::time::Duration::from_secs(3)).await;

        // Wait for the background task to complete
        let result = handle.await.unwrap();

        assert!(result.is_ok());
        let exec_result = result.unwrap();
        assert_eq!(exec_result.model_used, "model-1");
    }
}
