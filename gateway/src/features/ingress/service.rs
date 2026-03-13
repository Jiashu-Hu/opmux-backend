// Service Layer - Business logic and orchestration

use super::{
    constants::{AI_RESPONSE_ROLE, SLOW_REQUEST_THRESHOLD_MS},
    error::IngressError,
    repository::IngressRepository,
};
use crate::core::correlation::RequestContext;
use crate::features::executor::service::ExecutorService;
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
    slow_request_threshold_ms: u64,
}

impl IngressService {
    /// Creates a new ingress service instance with ExecutorService dependency.
    ///
    /// # Parameters
    /// - `executor_service` - Shared ExecutorService instance for LLM execution
    pub fn new(executor_service: Arc<ExecutorService>) -> Self {
        let slow_request_threshold_ms =
            std::env::var("INGRESS_SLOW_REQUEST_THRESHOLD_MS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(SLOW_REQUEST_THRESHOLD_MS);

        Self {
            repository: IngressRepository::new(executor_service),
            slow_request_threshold_ms,
        }
    }

    /// Processes an AI routing request through the complete AI pipeline.
    ///
    /// This is a CHILD SPAN. It automatically inherits `request_id` and
    /// `client_correlation_id` from the parent Handler span.
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
    /// - `request_context` - Request context with correlation IDs for gRPC calls
    ///
    /// # Returns
    /// Complete AI response with metadata (cost, model, processing time)
    #[tracing::instrument(
        skip(self, request, request_context),
        fields(
            user_id = %user_id,
            prompt_length = request.prompt.len(),
        )
    )]
    pub async fn process_request(
        &self,
        request: IngressRequest,
        user_id: String,
        request_context: &RequestContext,
    ) -> Result<IngressResponse, IngressError> {
        tracing::debug!("Starting request processing");
        let start_time = std::time::Instant::now();

        // Step 1: Get conversation context from Memory Service
        tracing::debug!("Retrieving conversation context from Memory Service");
        let context = self
            .repository
            .get_context(&user_id, request_context)
            .await
            .map_err(|_| IngressError::ContextRetrievalFailed)?;
        tracing::debug!("Context retrieved successfully");

        // Step 2: Build payload
        tracing::debug!("Building request payload");
        let payload = serde_json::json!({
            "messages": [
                {
                    "role": "user",
                    "content": request.prompt,
                }
            ],
            "metadata": request.metadata,
        });

        // Step 2.5: [FUTURE] Call RewriteService if metadata.rewrite=true
        // TODO: Implement conditional rewrite logic when RewriteService is available
        // if request.metadata.get("rewrite").and_then(|v| v.as_bool()).unwrap_or(false) {
        //     payload = self.repository.rewrite_request(&payload, &context, request_context).await?;
        // }

        // Step 3: Optimize route via Router Service
        tracing::debug!("Optimizing route via Router Service");
        let _router_response = self
            .repository
            .optimize_route(&payload, &context, request_context)
            .await
            .map_err(|_| IngressError::RequestOrchestrationFailed)?;
        tracing::debug!(
            vendor = %_router_response.optimized_plan.vendor_id,
            model = %_router_response.optimized_plan.model_id,
            "Route optimization completed"
        );

        // Step 4: Execute LLM call via ExecutorService with retry and fallback logic
        tracing::debug!("Executing LLM call via ExecutorService");
        let llm_result = self
            .repository
            .execute_llm_call(&_router_response.optimized_plan, &payload)
            .await?; // Use ? operator for automatic ExecutorError → IngressError conversion
        tracing::debug!(
            tokens = llm_result.prompt_tokens + llm_result.completion_tokens,
            cost = llm_result.total_cost,
            "LLM execution completed"
        );

        // Step 5: Update conversation context in Memory Service
        tracing::debug!("Updating conversation context in Memory Service");
        self.repository
            .update_context(
                &user_id,
                &request.prompt,
                &llm_result.content,
                request_context,
            )
            .await
            .map_err(|_| IngressError::ContextUpdateFailed)?;
        tracing::debug!("Context updated successfully");

        // Step 6: Calculate processing time and return response
        let processing_time_ms = start_time.elapsed().as_millis() as u64;
        if processing_time_ms >= self.slow_request_threshold_ms {
            tracing::warn!(
                processing_time_ms = processing_time_ms,
                threshold_ms = self.slow_request_threshold_ms,
                user_id = %user_id,
                "Slow ingress request detected"
            );
        }
        tracing::debug!(
            processing_time_ms = processing_time_ms,
            "Request processing completed"
        );

        Ok(IngressResponse {
            response: AIResponse {
                content: llm_result.content,
                role: AI_RESPONSE_ROLE.to_string(),
                finish_reason: Some(llm_result.finish_reason),
            },
            model_used: llm_result.model_used,
            cost: llm_result.total_cost,
            processing_time_ms,
        })
    }
}
