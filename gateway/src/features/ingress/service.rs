// Service Layer - Business logic and orchestration

use super::{
    constants::{AI_RESPONSE_ROLE, FINISH_REASON_STOP, REWRITE_PREFIX},
    error::IngressError,
    repository::IngressRepository,
};
use serde::{Deserialize, Serialize};

/// Incoming chat request from clients.
#[derive(Deserialize)]
pub struct IngressRequest {
    /// User's chat prompt/message.
    pub prompt: String,
    /// Additional request metadata (rewrite flags, preferences, etc.).
    pub metadata: serde_json::Value,
}

/// AI assistant response structure.
#[derive(Serialize)]
pub struct AIResponse {
    /// AI-generated response content.
    pub content: String,
    /// Response role (always "assistant").
    pub role: String,
    /// Reason for response completion ("stop", "length", etc.).
    pub finish_reason: Option<String>,
}

/// Complete ingress response with AI content and metadata.
#[derive(Serialize)]
pub struct IngressResponse {
    /// AI assistant response.
    pub response: AIResponse,
    /// AI model used for generation.
    pub model_used: String,
    /// Request cost in USD.
    pub cost: f64,
    /// Whether response came from cache.
    pub cache_hit: bool,
    /// Total processing time in milliseconds.
    pub processing_time_ms: u64,
}

/// Service for ingress request processing and microservice orchestration.
///
/// Coordinates the entire chat request flow: context retrieval, prompt processing,
/// AI routing, and response aggregation.
pub struct IngressService {
    repository: IngressRepository,
}

impl IngressService {
    /// Creates a new ingress service instance.
    pub fn new() -> Self {
        Self { repository: IngressRepository::new() }
    }

    /// Processes a chat request through the complete AI pipeline.
    ///
    /// # Flow
    /// 1. Retrieves conversation context from Memory Service
    /// 2. Processes prompt (applies rewrite if requested)
    /// 3. Routes to AI services via Router Service
    /// 4. Updates conversation context with new exchange
    ///
    /// # Parameters
    /// - `request` - Chat request with prompt and metadata
    /// - `user_id` - User identifier for context management
    ///
    /// # Returns
    /// Complete AI response with metadata (cost, model, processing time)
    pub async fn process_request(
        &self,
        request: IngressRequest,
        user_id: String,
    ) -> Result<IngressResponse, IngressError> {
        let start_time = std::time::Instant::now();

        // Step 1: Get conversation context from Memory Service
        let context = self
            .repository
            .get_context(&user_id)
            .await
            .map_err(|_| IngressError::ContextRetrievalFailed)?;

        // Step 2: Check if rewrite is needed (based on metadata)
        let processed_prompt = self.process_prompt(&request.prompt, &request.metadata).await?;

        // Step 3: Route request to Router Service with context
        let router_response = self
            .repository
            .route_request(&processed_prompt, &context, &request.metadata)
            .await
            .map_err(|_| IngressError::RequestOrchestrationFailed)?;

        // Step 4: Update conversation context in Memory Service
        self.repository
            .update_context(&user_id, &request.prompt, &router_response.ai_response)
            .await
            .map_err(|_| IngressError::ContextUpdateFailed)?;

        // Step 5: Calculate processing time and return response
        let processing_time = start_time.elapsed().as_millis() as u64;

        Ok(IngressResponse {
            response: AIResponse {
                content: router_response.ai_response,
                role: AI_RESPONSE_ROLE.to_string(),
                finish_reason: Some(FINISH_REASON_STOP.to_string()),
            },
            model_used: router_response.model_used,
            cost: router_response.cost,
            cache_hit: router_response.cache_hit,
            processing_time_ms: processing_time,
        })
    }

    /// Processes prompt with optional rewriting based on metadata.
    ///
    /// Checks `metadata.rewrite` flag and applies rewrite prefix if true.
    /// Future: Will integrate with Rewrite Service for advanced optimization.
    ///
    /// # Parameters
    /// - `prompt` - Original user prompt
    /// - `metadata` - Request metadata containing rewrite flag
    ///
    /// # Returns
    /// Processed prompt (with rewrite prefix if applicable)
    async fn process_prompt(
        &self,
        prompt: &str,
        metadata: &serde_json::Value,
    ) -> Result<String, IngressError> {
        // Check if rewrite is requested in metadata
        let needs_rewrite = metadata.get("rewrite").and_then(|v| v.as_bool()).unwrap_or(false);

        if needs_rewrite {
            // Future: call Rewrite Service via repository
            // For now, just return the original prompt with rewrite prefix
            Ok(format!("{} {}", REWRITE_PREFIX, prompt))
        } else {
            Ok(prompt.to_string())
        }
    }
}
