// Mock data definitions for ingress module
// This file centralizes all mock data used during development
// In production, this will be replaced with real service responses

use super::repository::{
    ContextData, LLMExecutionResult, RoutePlan, RouterServiceResponse,
};

/// Mock data provider for development and testing.
///
/// Provides realistic mock responses for all microservices during development.
/// Will be replaced with real gRPC service calls in production.
pub struct MockDataProvider;

impl MockDataProvider {
    /// Returns mock user ID for development/testing.
    ///
    /// In production, user ID will be extracted from JWT token.
    pub fn get_mock_user_id() -> String {
        "mock_user_123".to_string()
    }

    /// Returns mock conversation context from Memory Service.
    pub fn get_mock_context() -> ContextData {
        ContextData {
            conversation_history: vec![
                "User: Hello, how are you?".to_string(),
                "Assistant: I'm doing well, thank you for asking!".to_string(),
                "User: Can you help me with a coding question?".to_string(),
                "Assistant: Of course! I'd be happy to help with your coding question."
                    .to_string(),
            ],
            user_preferences:
                "casual tone, detailed explanations, code examples preferred".to_string(),
        }
    }

    /// Returns mock Router Service response (simple).
    ///
    /// # Returns
    /// Mock routing strategy with gpt-4 as default choice
    pub fn get_mock_router_response() -> RouterServiceResponse {
        RouterServiceResponse {
            optimized_plan: RoutePlan {
                vendor_id: "openai".to_string(),
                model_id: "gpt-4".to_string(),
                fallback_plans: vec![], // Empty in MVP
            },
            optimization_reason: "Selected gpt-4 for best quality".to_string(),
        }
    }

    /// Returns mock LLM execution result (simple).
    ///
    /// # Parameters
    /// - `plan` - Routing plan from Router Service
    /// - `payload` - Request payload
    ///
    /// # Returns
    /// Mock LLM execution result with fixed cost
    pub fn get_mock_llm_execution(
        plan: &RoutePlan,
        payload: &serde_json::Value,
    ) -> LLMExecutionResult {
        let prompt = payload
            .get("prompt")
            .and_then(|v| v.as_str())
            .unwrap_or("default prompt");

        LLMExecutionResult {
            ai_response: format!(
                "This is a mock AI response for: '{}' using model: {}",
                prompt, plan.model_id
            ),
            model_used: plan.model_id.clone(),
            actual_cost: 0.025, // Simple fixed value
        }
    }
}
