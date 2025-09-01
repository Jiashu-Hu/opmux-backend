// Mock data definitions for ingress module
// This file centralizes all mock data used during development
// In production, this will be replaced with real service responses

use super::repository::{ContextData, RouterResponse};

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
                "Assistant: Of course! I'd be happy to help with your coding question.".to_string(),
            ],
            user_preferences: "casual tone, detailed explanations, code examples preferred"
                .to_string(),
        }
    }

    /// Returns mock AI response from Router Service with cache hit.
    ///
    /// Generates response based on prompt content, detects rewrite prefix.
    ///
    /// # Parameters
    /// - `prompt` - User prompt (may include rewrite prefix)
    ///
    /// # Returns
    /// Mock router response with default cost and cache hit
    pub fn get_mock_router_response(prompt: &str) -> RouterResponse {
        let response_content = if prompt.contains("[Rewritten]") {
            format!("This is a mock AI response to your rewritten prompt: '{}'", prompt)
        } else {
            format!("This is a mock AI response to your prompt: '{}'", prompt)
        };

        RouterResponse {
            ai_response: response_content,
            model_used: "gpt-4".to_string(),
            cost: 0.002,
            cache_hit: true,
        }
    }

    /// Returns mock AI response from Router Service with cache miss (higher cost).
    pub fn get_mock_router_response_cache_miss(prompt: &str) -> RouterResponse {
        RouterResponse {
            ai_response: format!("This is a fresh AI response (cache miss) to: '{}'", prompt),
            model_used: "gpt-4".to_string(),
            cost: 0.005, // Higher cost for cache miss
            cache_hit: false,
        }
    }

    /// Returns mock AI response using premium model (highest cost).
    pub fn get_mock_router_response_expensive(prompt: &str) -> RouterResponse {
        RouterResponse {
            ai_response: format!(
                "This is a premium AI response using advanced model: '{}'",
                prompt
            ),
            model_used: "gpt-4-turbo".to_string(),
            cost: 0.010,
            cache_hit: false,
        }
    }
}
