// Service Layer - Business logic and orchestration

use super::{
    constants::{AI_RESPONSE_ROLE, FINISH_REASON_STOP},
    error::IngressError,
    repository::IngressRepository,
};
use crate::executor::service::ExecutorService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Incoming AI routing request from clients.
#[derive(Deserialize)]
pub struct IngressRequest {
    /// User's prompt/message for AI processing.
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
    /// Total processing time in milliseconds.
    pub processing_time_ms: u64,
}

/// Service for ingress request processing and microservice orchestration.
///
/// Coordinates the entire AI routing request flow: context retrieval, prompt processing,
/// AI routing, and response aggregation.
pub struct IngressService {
    repository: IngressRepository,
}

impl IngressService {
    /// Creates a new ingress service instance with ExecutorService dependency.
    ///
    /// # Parameters
    /// - `executor_service` - Shared ExecutorService instance for LLM execution
    pub fn new(executor_service: Arc<ExecutorService>) -> Self {
        Self {
            repository: IngressRepository::new(executor_service),
        }
    }

    /// Processes an AI routing request through the complete AI pipeline.
    ///
    /// # Flow
    /// 1. Retrieves conversation context from Memory Service
    /// 2. Builds request payload
    /// 3. Optimizes routing strategy via Router Service
    /// 4. Executes LLM call based on routing plan
    /// 5. Updates conversation context with new exchange
    ///
    /// # Parameters
    /// - `request` - AI routing request with prompt and metadata
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

        // Step 2: Build payload
        let payload = serde_json::json!({
            "prompt": request.prompt,
            "metadata": request.metadata,
        });

        // Step 2.5: [FUTURE] Call RewriteService if metadata.rewrite=true
        // TODO: Implement conditional rewrite logic when RewriteService is available
        // if request.metadata.get("rewrite").and_then(|v| v.as_bool()).unwrap_or(false) {
        //     payload = self.repository.rewrite_request(&payload, &context).await?;
        // }

        // Step 3: Optimize route via Router Service
        let _router_response = self
            .repository
            .optimize_route(&payload, &context)
            .await
            .map_err(|_| IngressError::RequestOrchestrationFailed)?;

        // Step 4: Execute LLM call via ExecutorService with retry and fallback logic
        let llm_result = self
            .repository
            .execute_llm_call(&_router_response.optimized_plan, &payload)
            .await?; // Use ? operator for automatic ExecutorError → IngressError conversion

        // Step 5: Update conversation context in Memory Service
        self.repository
            .update_context(&user_id, &request.prompt, &llm_result.ai_response)
            .await
            .map_err(|_| IngressError::ContextUpdateFailed)?;

        // Step 6: Calculate processing time and return response
        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(IngressResponse {
            response: AIResponse {
                content: llm_result.ai_response,
                role: AI_RESPONSE_ROLE.to_string(),
                finish_reason: Some(FINISH_REASON_STOP.to_string()),
            },
            model_used: llm_result.model_used,
            cost: llm_result.actual_cost,
            processing_time_ms,
        })
    }
}
