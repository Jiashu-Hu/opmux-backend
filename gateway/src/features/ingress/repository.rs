// Repository Layer - gRPC client management & mocks

use super::{error::IngressError, mockdata::MockDataProvider};

// Data structures returned by repository (simulating gRPC responses)
#[derive(Debug, Clone)]
pub struct ContextData {
    pub conversation_history: Vec<String>,
    pub user_preferences: String,
}

#[derive(Debug, Clone)]
pub struct RouterResponse {
    pub ai_response: String,
    pub model_used: String,
    pub cost: f64,
    pub cache_hit: bool,
}

// Repository struct - handles data access and gRPC client management
pub struct IngressRepository;

impl IngressRepository {
    pub fn new() -> Self {
        Self
    }

    // Mock Memory Service - retrieve conversation context
    pub async fn get_context(&self, _user_id: &str) -> Result<ContextData, IngressError> {
        // Mock implementation - in real version this would be gRPC call to Memory Service
        // In real implementation, gRPC failures would be mapped to ContextRetrievalFailed
        Ok(MockDataProvider::get_mock_context())
    }

    // Mock Router Service - process request with context
    pub async fn route_request(
        &self,
        prompt: &str,
        _context: &ContextData,
        _metadata: &serde_json::Value,
    ) -> Result<RouterResponse, IngressError> {
        // Mock implementation - in real version this would be gRPC call to Router Service
        // In real implementation, gRPC failures would be mapped to RequestOrchestrationFailed
        Ok(MockDataProvider::get_mock_router_response(prompt))
    }

    // Mock Memory Service - store updated context
    pub async fn update_context(
        &self,
        _user_id: &str,
        _new_message: &str,
        _response: &str,
    ) -> Result<(), IngressError> {
        // Mock implementation - in real version this would be gRPC call to Memory Service
        // In real implementation, gRPC failures would be mapped to ContextUpdateFailed
        Ok(())
    }
}
