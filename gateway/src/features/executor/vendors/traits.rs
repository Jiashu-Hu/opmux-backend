//! Common traits for LLM vendor implementations.

use crate::features::executor::{
    error::ExecutorError, models::ExecutionParams, models::ExecutionResult,
};
use async_trait::async_trait;

/// Trait for LLM vendor implementations.
///
/// Each vendor (OpenAI, Anthropic, etc.) implements this trait to provide
/// a unified interface for LLM execution.
#[async_trait]
pub trait LLMVendor: Send + Sync {
    /// Executes LLM API call with given parameters.
    ///
    /// # Parameters
    /// - `model` - Model identifier (e.g., "gpt-4", "claude-3-opus")
    /// - `params` - Execution parameters (messages, temperature, etc.)
    ///
    /// # Returns
    /// Execution result with AI response and metrics
    async fn execute(
        &self,
        model: &str,
        params: ExecutionParams,
    ) -> Result<ExecutionResult, ExecutorError>;

    /// Returns the vendor identifier.
    fn vendor_id(&self) -> &str;

    /// Checks if the vendor supports the given model.
    fn supports_model(&self, model: &str) -> bool;

    /// Calculates cost based on token usage.
    ///
    /// # Parameters
    /// - `prompt_tokens` - Number of tokens in the prompt
    /// - `completion_tokens` - Number of tokens in the completion
    /// - `model` - Model identifier
    ///
    /// # Returns
    /// Total cost in USD
    fn calculate_cost(
        &self,
        prompt_tokens: i64,
        completion_tokens: i64,
        model: &str,
    ) -> f64;
}
