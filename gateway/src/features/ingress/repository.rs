// Repository Layer - gRPC client management & mocks

use super::{
    constants::{CONTEXT_CACHE_MAX_ENTRIES, CONTEXT_CACHE_TTL_SECS},
    error::IngressError,
    mockdata::MockDataProvider,
};
use crate::core::contracts::RoutePlan;
use crate::core::correlation::RequestContext;
use crate::features::executor::{models::ExecutionResult, service::ExecutorService};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// User conversation context from Memory Service.
#[derive(Debug, Clone)]
pub struct ContextData {
    /// Previous conversation messages.
    pub conversation_history: Vec<String>,
    /// User's AI preferences and settings.
    pub user_preferences: String,
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
    context_cache: Arc<RwLock<HashMap<String, CachedContext>>>,
    context_cache_ttl: Duration,
    context_cache_max_entries: usize,
}

#[derive(Debug, Clone)]
struct CachedContext {
    data: ContextData,
    cached_at: Instant,
}

impl IngressRepository {
    /// Creates a new repository instance with ExecutorService dependency.
    ///
    /// # Parameters
    /// - `executor_service` - Shared ExecutorService instance for LLM execution
    pub fn new(executor_service: Arc<ExecutorService>) -> Self {
        Self {
            executor_service,
            context_cache: Arc::new(RwLock::new(HashMap::new())),
            context_cache_ttl: Duration::from_secs(CONTEXT_CACHE_TTL_SECS),
            context_cache_max_entries: CONTEXT_CACHE_MAX_ENTRIES,
        }
    }

    /// Retrieves user conversation context from Memory Service.
    ///
    /// # Parameters
    /// - `user_id` - User identifier for context lookup
    /// - `request_context` - Request context with correlation IDs for gRPC metadata
    ///
    /// # Returns
    /// User's conversation history and preferences
    pub async fn get_context(
        &self,
        user_id: &str,
        _request_context: &RequestContext,
    ) -> Result<ContextData, IngressError> {
        {
            let cache = self.context_cache.read().await;
            if let Some(cached) = cache.get(user_id) {
                if cached.cached_at.elapsed() < self.context_cache_ttl {
                    tracing::debug!(
                        user_id = %user_id,
                        ttl_secs = self.context_cache_ttl.as_secs(),
                        "Context cache hit"
                    );
                    return Ok(cached.data.clone());
                }
            }
        }

        // Mock implementation - real version will use gRPC to Memory Service
        // Future: Build gRPC RequestMeta from request_context
        // let grpc_request = GetContextRequest {
        //     meta: Some(RequestMeta {
        //         request_id: request_context.request_id.clone(),
        //         client_id: user_id.to_string(),
        //         traceparent: /* ... */,
        //         deadline_ms: /* ... */,
        //     }),
        //     user_id: user_id.to_string(),
        // };
        // gRPC failures mapped to ContextRetrievalFailed
        let context = MockDataProvider::get_mock_context();
        {
            let mut cache = self.context_cache.write().await;
            cache.retain(|_, entry| entry.cached_at.elapsed() < self.context_cache_ttl);
            if cache.len() >= self.context_cache_max_entries {
                if let Some(evict_key) = cache
                    .iter()
                    .min_by_key(|(_, entry)| entry.cached_at)
                    .map(|(key, _)| key.clone())
                {
                    cache.remove(&evict_key);
                }
            }
            cache.insert(
                user_id.to_string(),
                CachedContext {
                    data: context.clone(),
                    cached_at: Instant::now(),
                },
            );
        }
        tracing::debug!(user_id = %user_id, "Context cache miss");
        Ok(context)
    }

    /// Optimizes routing strategy via Router Service.
    ///
    /// This is a CHILD SPAN. It automatically inherits `request_id` from parent.
    ///
    /// Determines the best routing plan based on request payload.
    /// Does NOT execute the actual LLM call.
    ///
    /// # Parameters
    /// - `payload` - Complete request payload
    /// - `context` - User conversation context
    /// - `request_context` - Request context with correlation IDs for gRPC metadata
    ///
    /// # Returns
    /// Router Service response with optimized routing plan
    #[tracing::instrument(skip(self, _payload, _context, _request_context))]
    pub async fn optimize_route(
        &self,
        _payload: &serde_json::Value,
        _context: &ContextData,
        _request_context: &RequestContext,
    ) -> Result<RouterServiceResponse, IngressError> {
        tracing::debug!("Calling Router Service for route optimization");
        // Mock implementation - real version will use gRPC to Router Service
        // Future: Build gRPC RequestMeta from request_context
        // let grpc_request = OptimizeRouteRequest {
        //     meta: Some(RequestMeta {
        //         request_id: request_context.request_id.clone(),
        //         client_id: /* from auth_context */,
        //         traceparent: /* ... */,
        //         deadline_ms: /* ... */,
        //     }),
        //     original_payload: Some(payload.clone()),
        //     context: /* ... */,
        // };
        // gRPC failures mapped to RequestOrchestrationFailed
        let response = MockDataProvider::get_mock_router_response();
        tracing::debug!(
            vendor = %response.optimized_plan.vendor_id,
            model = %response.optimized_plan.model_id,
            reason = %response.optimization_reason,
            "Router Service returned optimization plan"
        );
        Ok(response)
    }

    /// Executes LLM call based on routing plan using ExecutorService.
    ///
    /// This is a CHILD SPAN. It automatically inherits `request_id` from parent.
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
    #[tracing::instrument(
        skip(self, payload),
        fields(
            vendor_id = %plan.vendor_id,
            model_id = %plan.model_id,
        )
    )]
    pub async fn execute_llm_call(
        &self,
        plan: &RoutePlan,
        payload: &serde_json::Value,
    ) -> Result<ExecutionResult, IngressError> {
        tracing::debug!("Executing LLM call via ExecutorService");
        // Execute via ExecutorService with retry and fallback logic
        // ExecutorError is automatically converted to IngressError via #[from]
        let result = self
            .executor_service
            .execute(plan, payload)
            .await
            .map_err(IngressError::from)?;

        tracing::debug!(
            prompt_tokens = result.prompt_tokens,
            completion_tokens = result.completion_tokens,
            total_cost = result.total_cost,
            "LLM call completed successfully"
        );

        Ok(result)
    }

    /// Updates conversation context in Memory Service.
    ///
    /// Stores the new user message and AI response in conversation history.
    ///
    /// # Parameters
    /// - `user_id` - User identifier for context storage
    /// - `new_message` - User's original message
    /// - `response` - AI's generated response
    /// - `request_context` - Request context with correlation IDs for gRPC metadata
    pub async fn update_context(
        &self,
        user_id: &str,
        _new_message: &str,
        _response: &str,
        _request_context: &RequestContext,
    ) -> Result<(), IngressError> {
        // Mock implementation - real version will use gRPC to Memory Service
        // Future: Build gRPC RequestMeta from request_context
        // let grpc_request = UpdateContextRequest {
        //     meta: Some(RequestMeta {
        //         request_id: request_context.request_id.clone(),
        //         client_id: user_id.to_string(),
        //         traceparent: /* ... */,
        //         deadline_ms: /* ... */,
        //     }),
        //     user_id: user_id.to_string(),
        //     new_message: new_message.to_string(),
        //     response: response.to_string(),
        // };
        // gRPC failures mapped to ContextUpdateFailed
        {
            let mut cache = self.context_cache.write().await;
            cache.remove(user_id);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::executor::{
        config::ExecutorConfig,
        error::ExecutorError,
        models::{ExecutionParams, ExecutionResult},
        repository::ExecutorRepository,
        service::ExecutorService,
        vendors::LLMVendor,
    };
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[derive(Clone)]
    struct MockVendor {
        vendor_id: String,
    }

    #[async_trait]
    impl LLMVendor for MockVendor {
        async fn execute(
            &self,
            model: &str,
            _params: ExecutionParams,
        ) -> Result<ExecutionResult, ExecutorError> {
            Ok(ExecutionResult {
                content: "ok".to_string(),
                model_used: model.to_string(),
                prompt_tokens: 1,
                completion_tokens: 1,
                total_cost: 0.0,
                finish_reason: "stop".to_string(),
            })
        }

        fn vendor_id(&self) -> &str {
            &self.vendor_id
        }

        fn supports_model(&self, _model: &str) -> bool {
            true
        }

        fn calculate_cost(
            &self,
            _prompt_tokens: i64,
            _completion_tokens: i64,
            _model: &str,
        ) -> f64 {
            0.0
        }

        async fn health_check(&self, _timeout_secs: u64) -> Result<(), ExecutorError> {
            Ok(())
        }
    }

    fn create_repository() -> IngressRepository {
        let mut vendors: HashMap<String, Arc<dyn LLMVendor>> = HashMap::new();
        vendors.insert(
            "openai".to_string(),
            Arc::new(MockVendor {
                vendor_id: "openai".to_string(),
            }),
        );

        let executor = Arc::new(ExecutorService {
            repository: Arc::new(ExecutorRepository { vendors }),
            config: ExecutorConfig {
                openai: None,
                anthropic_api_key: None,
                timeout_ms: 30000,
                max_retries: 0,
            },
            circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
            circuit_breaker_failure_threshold: 3,
            circuit_breaker_open_duration: Duration::from_secs(30),
        });

        IngressRepository::new(executor)
    }

    #[tokio::test]
    async fn test_get_context_populates_cache_for_user() {
        let repository = create_repository();
        let request_context = RequestContext::new("req-1".to_string(), None);

        let _ = repository
            .get_context("user-a", &request_context)
            .await
            .expect("context retrieval should succeed");

        let cache = repository.context_cache.read().await;
        assert!(cache.contains_key("user-a"));
    }

    #[tokio::test]
    async fn test_get_context_refreshes_expired_cache_entry() {
        let mut repository = create_repository();
        repository.context_cache_ttl = Duration::from_millis(10);
        let request_context = RequestContext::new("req-2".to_string(), None);

        let _ = repository
            .get_context("user-b", &request_context)
            .await
            .expect("first context retrieval should succeed");

        tokio::time::sleep(Duration::from_millis(20)).await;

        let _ = repository
            .get_context("user-b", &request_context)
            .await
            .expect("second context retrieval should succeed after ttl expiry");

        let cache = repository.context_cache.read().await;
        assert!(cache.contains_key("user-b"));
    }

    #[tokio::test]
    async fn test_get_context_evicts_oldest_when_cache_is_full() {
        let mut repository = create_repository();
        repository.context_cache_max_entries = 1;
        let request_context = RequestContext::new("req-3".to_string(), None);

        let _ = repository
            .get_context("user-1", &request_context)
            .await
            .expect("first context retrieval should succeed");

        tokio::time::sleep(Duration::from_millis(2)).await;

        let _ = repository
            .get_context("user-2", &request_context)
            .await
            .expect("second context retrieval should succeed");

        let cache = repository.context_cache.read().await;
        assert!(!cache.contains_key("user-1"));
        assert!(cache.contains_key("user-2"));
        assert_eq!(cache.len(), 1);
    }

    #[tokio::test]
    async fn test_update_context_invalidates_cached_user_context() {
        let repository = create_repository();
        let request_context = RequestContext::new("req-4".to_string(), None);

        let _ = repository
            .get_context("user-c", &request_context)
            .await
            .expect("context retrieval should succeed");

        {
            let cache = repository.context_cache.read().await;
            assert!(cache.contains_key("user-c"));
        }

        repository
            .update_context("user-c", "hello", "world", &request_context)
            .await
            .expect("update_context should succeed");

        let cache = repository.context_cache.read().await;
        assert!(!cache.contains_key("user-c"));
    }
}
