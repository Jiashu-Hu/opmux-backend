// Repository Layer - gRPC client management & mocks

use super::{error::IngressError, mockdata::MockDataProvider};
use crate::features::executor::{models::ExecutionResult, service::ExecutorService};
use std::sync::Arc;

/// User conversation context from Memory Service.
#[derive(Debug, Clone)]
pub struct ContextData {
    /// Previous conversation messages.
    pub conversation_history: Vec<String>,
    /// User's AI preferences and settings.
    pub user_preferences: String,
}

/// Routing strategy plan from Router Service.
#[derive(Debug, Clone)]
pub struct RoutePlan {
    /// Vendor identifier (e.g., "openai", "anthropic", "cohere")
    pub vendor_id: String,
    /// Model identifier (e.g., "gpt-4", "claude-3-opus")
    pub model_id: String,
    /// Fallback strategy chain (empty in MVP)
    pub fallback_plans: Vec<RoutePlan>,
}

/// Router Service response with routing strategy.
#[derive(Debug, Clone)]
pub struct RouterServiceResponse {
    /// Optimized routing plan
    pub optimized_plan: RoutePlan,
    /// Why this strategy was chosen (for debugging/monitoring)
    pub optimization_reason: String,
}

/// Repository for microservice communication and data access.
///
/// Manages gRPC clients for Memory, Router, Rewrite, and Validation services.
/// Uses ExecutorService for LLM execution.
pub struct IngressRepository {
    /// Executor service for LLM execution with retry and fallback logic
    executor_service: Arc<ExecutorService>,
}

impl IngressRepository {
    /// Creates a new repository instance with ExecutorService dependency.
    ///
    /// # Parameters
    /// - `executor_service` - Shared ExecutorService instance for LLM execution
    pub fn new(executor_service: Arc<ExecutorService>) -> Self {
        Self { executor_service }
    }

    /// Retrieves user conversation context from Memory Service.
    ///
    /// # Parameters
    /// - `user_id` - User identifier for context lookup
    ///
    /// # Returns
    /// User's conversation history and preferences
    pub async fn get_context(&self, _user_id: &str) -> Result<ContextData, IngressError> {
        // Mock implementation - real version will use gRPC to Memory Service
        // gRPC failures mapped to ContextRetrievalFailed
        Ok(MockDataProvider::get_mock_context())
    }

    /// Optimizes routing strategy via Router Service.
    ///
    /// Determines the best routing plan based on request payload.
    /// Does NOT execute the actual LLM call.
    ///
    /// # Parameters
    /// - `payload` - Complete request payload
    /// - `context` - User conversation context
    ///
    /// # Returns
    /// Router Service response with optimized routing plan
    pub async fn optimize_route(
        &self,
        _payload: &serde_json::Value,
        _context: &ContextData,
    ) -> Result<RouterServiceResponse, IngressError> {
        // Mock implementation - real version will use gRPC to Router Service
        // gRPC failures mapped to RequestOrchestrationFailed
        Ok(MockDataProvider::get_mock_router_response())
    }

    /// Executes LLM call based on routing plan using ExecutorService.
    ///
    /// Delegates to ExecutorService which handles retry logic, fallback execution,
    /// and vendor-specific API calls.
    ///
    /// # Parameters
    /// - `plan` - Routing plan from Router Service
    /// - `payload` - Request payload to send to LLM
    ///
    /// # Returns
    /// ExecutionResult with response content, token counts, and cost metrics
    ///
    /// # Errors
    /// Returns ExecutionFailed if LLM execution fails (automatically converted from ExecutorError)
    pub async fn execute_llm_call(
        &self,
        plan: &RoutePlan,
        payload: &serde_json::Value,
    ) -> Result<ExecutionResult, IngressError> {
        // Execute via ExecutorService with retry and fallback logic
        // No conversion needed - directly return ExecutionResult
        self.executor_service
            .execute(plan, payload)
            .await
            .map_err(Into::into)
    }

    /// Updates conversation context in Memory Service.
    ///
    /// Stores the new user message and AI response in conversation history.
    ///
    /// # Parameters
    /// - `user_id` - User identifier for context storage
    /// - `new_message` - User's original message
    /// - `response` - AI's generated response
    pub async fn update_context(
        &self,
        _user_id: &str,
        _new_message: &str,
        _response: &str,
    ) -> Result<(), IngressError> {
        // Mock implementation - real version will use gRPC to Memory Service
        // gRPC failures mapped to ContextUpdateFailed
        Ok(())
    }

    // --- Future Methods (Not Implemented in MVP) ---

    // /// Rewrites request via Rewrite Service (future implementation).
    // ///
    // /// Applies prompt optimization and template transformations.
    // ///
    // /// # Parameters
    // /// - `payload` - Original request payload
    // /// - `context` - User conversation context
    // ///
    // /// # Returns
    // /// Rewritten request payload
    // pub async fn rewrite_request(
    //     &self,
    //     _payload: &serde_json::Value,
    //     _context: &ContextData,
    // ) -> Result<serde_json::Value, IngressError> {
    //     // Future: gRPC call to Rewrite Service
    //     // gRPC failures mapped to RequestOrchestrationFailed
    //     unimplemented!("RewriteService not yet implemented")
    // }
}
