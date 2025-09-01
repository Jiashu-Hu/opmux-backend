// Service Layer - Business logic and orchestration

use super::repository::IngressRepository;
use serde::{Deserialize, Serialize};

// Request/Response models
#[derive(Deserialize)]
pub struct IngressRequest {
    pub prompt: String,
    pub metadata: serde_json::Value,
}

#[derive(Serialize)]
pub struct AIResponse {
    pub content: String,
    pub role: String,                  // "assistant"
    pub finish_reason: Option<String>, // "stop", "length", etc.
}

#[derive(Serialize)]
pub struct IngressResponse {
    pub response: AIResponse,
    pub model_used: String,
    pub cost: f64,
    pub cache_hit: bool,
    pub processing_time_ms: u64,
}

// Service struct - handles business logic and orchestration
pub struct IngressService {
    repository: IngressRepository,
}

impl IngressService {
    pub fn new() -> Self {
        Self { repository: IngressRepository::new() }
    }

    // Main business logic - orchestrates the entire request flow
    pub async fn process_request(
        &self,
        request: IngressRequest,
        user_id: String,
    ) -> Result<IngressResponse, String> {
        let start_time = std::time::Instant::now();

        // Step 1: Get conversation context from Memory Service
        let context = self
            .repository
            .get_context(&user_id)
            .await
            .map_err(|e| format!("Failed to get context: {}", e))?;

        // Step 2: Check if rewrite is needed (based on metadata)
        let processed_prompt = self.process_prompt(&request.prompt, &request.metadata).await?;

        // Step 3: Route request to Router Service with context
        let router_response = self
            .repository
            .route_request(&processed_prompt, &context, &request.metadata)
            .await
            .map_err(|e| format!("Failed to route request: {}", e))?;

        // Step 4: Update conversation context in Memory Service
        self.repository
            .update_context(&user_id, &request.prompt, &router_response.ai_response)
            .await
            .map_err(|e| format!("Failed to update context: {}", e))?;

        // Step 5: Calculate processing time and return response
        let processing_time = start_time.elapsed().as_millis() as u64;

        Ok(IngressResponse {
            response: AIResponse {
                content: router_response.ai_response,
                role: "assistant".to_string(),
                finish_reason: Some("stop".to_string()),
            },
            model_used: router_response.model_used,
            cost: router_response.cost,
            cache_hit: router_response.cache_hit,
            processing_time_ms: processing_time,
        })
    }

    // Business logic for prompt processing (future: integrate with Rewrite Service)
    async fn process_prompt(
        &self,
        prompt: &str,
        metadata: &serde_json::Value,
    ) -> Result<String, String> {
        // Check if rewrite is requested in metadata
        let needs_rewrite = metadata.get("rewrite").and_then(|v| v.as_bool()).unwrap_or(false);

        if needs_rewrite {
            // Future: call Rewrite Service via repository
            // For now, just return the original prompt
            Ok(format!("[Rewritten] {}", prompt))
        } else {
            Ok(prompt.to_string())
        }
    }
}
