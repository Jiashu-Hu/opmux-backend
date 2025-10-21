// Service Layer - Business logic for LLM execution (retry, fallback, parameter extraction)

use super::{
    config::ExecutorConfig,
    error::ExecutorError,
    models::{ExecutionParams, ExecutionResult},
    repository::ExecutorRepository,
};
use crate::features::ingress::repository::RoutePlan;
use std::sync::Arc;

/// Service for LLM execution with business logic.
///
/// Handles retry logic, fallback execution, and parameter extraction.
/// Delegates direct API calls to ExecutorRepository.
pub struct ExecutorService {
    /// Repository for vendor management and direct API calls
    pub(crate) repository: Arc<ExecutorRepository>,
    /// Executor configuration for retry logic and timeout settings
    pub(crate) config: ExecutorConfig,
}

impl ExecutorService {
    /// Creates ExecutorService from configuration.
    ///
    /// # Parameters
    /// - `config` - Executor configuration with vendor settings
    ///
    /// # Returns
    /// ExecutorService instance with initialized repository
    ///
    /// # Errors
    /// Returns `NoVendorsConfigured` if no vendors are configured
    pub fn from_config(config: ExecutorConfig) -> Result<Self, ExecutorError> {
        let repository = ExecutorRepository::from_config(config.clone())?;
        Ok(Self {
            repository: Arc::new(repository),
            config,
        })
    }

    /// Returns the number of registered vendors.
    ///
    /// Useful for logging and monitoring vendor availability.
    pub fn vendor_count(&self) -> usize {
        self.repository.vendor_count()
    }

    /// Executes LLM call with retry logic and exponential backoff.
    ///
    /// # Flow
    /// 1. Validate model support via repository
    /// 2. Attempt execution with retry loop
    /// 3. Apply exponential backoff between retries (1s, 2s, 4s, 8s...)
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
    pub(crate) async fn execute_with_retry(
        &self,
        vendor_id: &str,
        model_id: &str,
        params: &ExecutionParams,
    ) -> Result<ExecutionResult, ExecutorError> {
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

            // Attempt execution via repository
            match self.repository.call_llm(vendor_id, model_id, params).await {
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
    pub(crate) fn is_retryable_error(error: &ExecutorError) -> bool {
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
    pub(crate) async fn execute_fallbacks(
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

        // Try each fallback plan sequentially
        for (index, fallback) in fallback_plans.iter().enumerate() {
            tracing::info!(
                "Attempting fallback {}/{}: vendor={}, model={}",
                index + 1,
                fallback_plans.len(),
                fallback.vendor_id,
                fallback.model_id
            );

            // Each fallback gets full retry logic
            match self
                .execute_with_retry(&fallback.vendor_id, &fallback.model_id, params)
                .await
            {
                Ok(result) => {
                    tracing::info!(
                        "Fallback {}/{} succeeded: vendor={}, model={}",
                        index + 1,
                        fallback_plans.len(),
                        fallback.vendor_id,
                        fallback.model_id
                    );
                    return Ok(result);
                }
                Err(e) => {
                    tracing::warn!(
                        "Fallback {}/{} failed: vendor={}, model={}, error={:?}",
                        index + 1,
                        fallback_plans.len(),
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
    pub(crate) fn extract_params(
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
