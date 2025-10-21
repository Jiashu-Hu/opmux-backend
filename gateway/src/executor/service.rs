//! Executor service layer - orchestrates LLM execution workflow.

use super::{
    config::ExecutorConfig,
    error::ExecutorError,
    models::ExecutionResult,
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
