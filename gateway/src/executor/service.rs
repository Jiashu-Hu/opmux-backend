//! Executor service layer - orchestrates LLM execution workflow.

use super::{error::ExecutorError, models::ExecutionResult};
use crate::features::ingress::repository::RoutePlan;

/// Service for executing LLM API calls based on routing decisions.
///
/// Manages vendor registration, parameter extraction, and execution orchestration.
pub struct ExecutorService {
    // Vendor registry will be added in Task 2.1
}

impl ExecutorService {
    /// Creates a new executor service instance.
    pub fn new() -> Self {
        Self {}
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
        // Implementation will be added in Task 3.1
        todo!("ExecutorService::execute not yet implemented")
    }
}

impl Default for ExecutorService {
    fn default() -> Self {
        Self::new()
    }
}
