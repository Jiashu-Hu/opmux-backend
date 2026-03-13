#[cfg(test)]
mod tests {
    use crate::core::correlation::RequestContext;
    use crate::features::executor::{
        config::ExecutorConfig,
        error::ExecutorError,
        models::{ExecutionParams, ExecutionResult},
        repository::ExecutorRepository,
        service::ExecutorService,
        vendors::LLMVendor,
    };
    use crate::features::ingress::{
        error::IngressError,
        service::{IngressRequest, IngressService},
    };
    use async_trait::async_trait;
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::RwLock;

    #[derive(Clone)]
    struct MockVendor {
        vendor_id: String,
        supported_models: Vec<String>,
    }

    impl MockVendor {
        fn new(vendor_id: &str, models: Vec<&str>) -> Self {
            Self {
                vendor_id: vendor_id.to_string(),
                supported_models: models.into_iter().map(|m| m.to_string()).collect(),
            }
        }
    }

    #[async_trait]
    impl LLMVendor for MockVendor {
        async fn execute(
            &self,
            model: &str,
            _params: ExecutionParams,
        ) -> Result<ExecutionResult, ExecutorError> {
            Ok(ExecutionResult {
                content: "Mock ingress response".to_string(),
                model_used: model.to_string(),
                prompt_tokens: 15,
                completion_tokens: 25,
                total_cost: 0.001,
                finish_reason: "stop".to_string(),
            })
        }

        fn vendor_id(&self) -> &str {
            &self.vendor_id
        }

        fn supports_model(&self, model: &str) -> bool {
            self.supported_models.contains(&model.to_string())
        }

        fn calculate_cost(
            &self,
            _prompt_tokens: i64,
            _completion_tokens: i64,
            _model: &str,
        ) -> f64 {
            0.001
        }

        async fn health_check(&self, _timeout_secs: u64) -> Result<(), ExecutorError> {
            Ok(())
        }
    }

    fn create_mock_executor_service(models: Vec<&str>) -> Arc<ExecutorService> {
        let vendor = MockVendor::new("openai", models);

        let mut vendors = HashMap::new();
        vendors.insert("openai".to_string(), Arc::new(vendor) as Arc<dyn LLMVendor>);

        let repository = ExecutorRepository { vendors };

        Arc::new(ExecutorService {
            repository: Arc::new(repository),
            config: ExecutorConfig {
                openai: None,
                anthropic_api_key: None,
                timeout_ms: 30000,
                max_retries: 3,
            },
            circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
            circuit_breaker_failure_threshold: 3,
            circuit_breaker_open_duration: Duration::from_secs(30),
        })
    }

    #[tokio::test]
    async fn test_process_request_success_returns_expected_response_shape() {
        let service = IngressService::new(create_mock_executor_service(vec!["gpt-4"]));
        let request_context = RequestContext::new(
            "req-ingress-1".to_string(),
            Some("client-1".to_string()),
        );

        let result = service
            .process_request(
                IngressRequest {
                    prompt: "Hello ingress".to_string(),
                    metadata: json!({ "rewrite": false }),
                },
                "user-123".to_string(),
                &request_context,
            )
            .await
            .expect("ingress request should succeed");

        assert_eq!(result.response.role, "assistant");
        assert_eq!(result.response.content, "Mock ingress response");
        assert_eq!(result.response.finish_reason, Some("stop".to_string()));
        assert_eq!(result.model_used, "gpt-4");
        assert_eq!(result.cost, 0.001);
        assert!(result.processing_time_ms < 5_000);
    }

    #[tokio::test]
    async fn test_process_request_returns_execution_failed_for_unsupported_model() {
        let service =
            IngressService::new(create_mock_executor_service(vec!["gpt-3.5-turbo"]));
        let request_context = RequestContext::new("req-ingress-2".to_string(), None);

        let result = service
            .process_request(
                IngressRequest {
                    prompt: "Trigger unsupported model".to_string(),
                    metadata: json!({}),
                },
                "user-456".to_string(),
                &request_context,
            )
            .await;

        match result {
            Err(IngressError::ExecutionFailed(ExecutorError::UnsupportedModel(
                model,
                vendor,
            ))) => {
                assert_eq!(model, "openai");
                assert_eq!(vendor, "gpt-4");
            }
            _ => panic!(
                "Expected UnsupportedModel wrapped by IngressError::ExecutionFailed"
            ),
        }
    }
}
