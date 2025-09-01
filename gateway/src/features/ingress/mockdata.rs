// Mock data definitions for ingress module
// This file centralizes all mock data used during development
// In production, this will be replaced with real service responses

use super::repository::{ContextData, RouterResponse};

/// Mock data provider for development and testing
pub struct MockDataProvider;

impl MockDataProvider {
    /// Get mock user ID for development/testing
    /// In production, this will be extracted from JWT token in the handler layer
    pub fn get_mock_user_id() -> String {
        "mock_user_123".to_string()
    }
    /// Get mock conversation context data
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

    /// Get mock router response for normal requests
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

    /// Get mock router response with cache miss
    pub fn get_mock_router_response_cache_miss(prompt: &str) -> RouterResponse {
        RouterResponse {
            ai_response: format!("This is a fresh AI response (cache miss) to: '{}'", prompt),
            model_used: "gpt-4".to_string(),
            cost: 0.005, // Higher cost for cache miss
            cache_hit: false,
        }
    }

    /// Get mock router response for expensive model
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
