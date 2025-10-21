//! Integration tests for Executor Layer with real OpenAI API.
//!
//! These tests require valid API credentials and make real API calls.
//!
//! # Environment Variables Required
//! - `OPENAI_API_KEY` - Valid OpenAI API key
//! - `OPENAI_BASE_URL` - API endpoint (optional, defaults to https://api.openai.com/v1)
//!
//! # Running Tests
//! ```bash
//! OPENAI_API_KEY=your-key OPENAI_BASE_URL=https://api.openai.com/v1 cargo test --test executor_integration_test -- --nocapture
//! ```

use gateway::features::{
    executor::{config::ExecutorConfig, service::ExecutorService},
    ingress::repository::RoutePlan,
};
use serde_json::json;

/// Helper to check if integration tests should run.
///
/// Integration tests are skipped if OPENAI_API_KEY is not set.
fn should_run_integration_tests() -> bool {
    std::env::var("OPENAI_API_KEY").is_ok()
}

/// Helper to create a simple RoutePlan for testing.
fn create_test_route_plan(vendor_id: &str, model_id: &str) -> RoutePlan {
    RoutePlan {
        vendor_id: vendor_id.to_string(),
        model_id: model_id.to_string(),
        fallback_plans: vec![],
    }
}

/// Helper to create test payload.
fn create_test_payload(content: &str) -> serde_json::Value {
    json!({
        "messages": [
            {"role": "user", "content": content}
        ],
        "temperature": 0.7,
        "max_tokens": 50
    })
}

#[tokio::test]
async fn test_openai_api_basic_execution() {
    if !should_run_integration_tests() {
        println!("⏭️  Skipping integration test: OPENAI_API_KEY not set");
        return;
    }

    // Load configuration from environment
    let config = ExecutorConfig::from_env();
    config.validate();

    // Create ExecutorService
    let service = ExecutorService::from_config(config)
        .expect("Failed to create ExecutorService from config");

    println!(
        "✅ ExecutorService initialized with {} vendors",
        service.vendor_count()
    );

    // Create route plan and payload
    let plan = create_test_route_plan("openai", "gpt-3.5-turbo");
    let payload = create_test_payload("Say 'Hello, World!' and nothing else.");

    // Execute with gpt-3.5-turbo (cheapest model for testing)
    let result = service.execute(&plan, &payload).await;

    match result {
        Ok(execution_result) => {
            println!("✅ API call successful!");
            println!("   Model: {}", execution_result.model_used);
            println!("   Response: {}", execution_result.content);
            println!("   Prompt tokens: {}", execution_result.prompt_tokens);
            println!(
                "   Completion tokens: {}",
                execution_result.completion_tokens
            );
            println!("   Total cost: ${:.6}", execution_result.total_cost);
            println!("   Finish reason: {}", execution_result.finish_reason);

            // Assertions
            assert_eq!(execution_result.model_used, "gpt-3.5-turbo");
            assert!(
                !execution_result.content.is_empty(),
                "Response should not be empty"
            );
            assert!(
                execution_result.prompt_tokens > 0,
                "Should have prompt tokens"
            );
            assert!(
                execution_result.completion_tokens > 0,
                "Should have completion tokens"
            );
            assert!(
                execution_result.total_cost >= 0.0,
                "Cost should be non-negative"
            );
            assert_eq!(execution_result.finish_reason, "stop");
        }
        Err(e) => {
            panic!("❌ API call failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_openai_api_with_different_models() {
    if !should_run_integration_tests() {
        println!("⏭️  Skipping integration test: OPENAI_API_KEY not set");
        return;
    }

    let config = ExecutorConfig::from_env();
    let service =
        ExecutorService::from_config(config).expect("Failed to create ExecutorService");

    let plan = create_test_route_plan("openai", "gpt-3.5-turbo");
    let payload = json!({
        "messages": [
            {"role": "user", "content": "Say 'Hello' and nothing else."}
        ],
        "temperature": 0.5,
        "max_tokens": 30
    });

    // Test with gpt-3.5-turbo
    let result = service.execute(&plan, &payload).await;

    assert!(result.is_ok(), "gpt-3.5-turbo should succeed");
    println!("✅ gpt-3.5-turbo test passed");

    // Test with gpt-4 (if available and budget allows)
    // Note: This is commented out by default to avoid high costs
    // Uncomment if you want to test gpt-4
    /*
    let plan_gpt4 = create_test_route_plan("openai", "gpt-4");
    let result_gpt4 = service.execute(&plan_gpt4, &payload).await;

    if result_gpt4.is_ok() {
        println!("✅ gpt-4 test passed");
    } else {
        println!("⚠️  gpt-4 test failed (may not be available): {:?}", result_gpt4.err());
    }
    */
}

#[tokio::test]
async fn test_openai_api_parameter_extraction() {
    if !should_run_integration_tests() {
        println!("⏭️  Skipping integration test: OPENAI_API_KEY not set");
        return;
    }

    let config = ExecutorConfig::from_env();
    let service =
        ExecutorService::from_config(config).expect("Failed to create ExecutorService");

    // Test parameter extraction from JSON payload
    let plan = create_test_route_plan("openai", "gpt-3.5-turbo");
    let payload = json!({
        "messages": [
            {"role": "user", "content": "What is 2+2?"}
        ],
        "temperature": 0.3,
        "max_tokens": 20
    });

    let result = service.execute(&plan, &payload).await;

    match result {
        Ok(execution_result) => {
            println!("✅ Parameter extraction and execution successful!");
            println!("   Response: {}", execution_result.content);
            assert!(!execution_result.content.is_empty());
        }
        Err(e) => {
            panic!("❌ Parameter extraction test failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_openai_api_retry_logic() {
    if !should_run_integration_tests() {
        println!("⏭️  Skipping integration test: OPENAI_API_KEY not set");
        return;
    }

    let config = ExecutorConfig::from_env();
    let service =
        ExecutorService::from_config(config).expect("Failed to create ExecutorService");

    let plan = create_test_route_plan("openai", "gpt-3.5-turbo");
    let payload = create_test_payload("Say 'Test' and nothing else.");

    // Execute - should succeed even if there are transient network issues
    // (retry logic will handle them)
    let result = service.execute(&plan, &payload).await;

    assert!(
        result.is_ok(),
        "Retry logic should handle transient failures"
    );
    println!("✅ Retry logic test passed");
}

#[tokio::test]
async fn test_openai_api_unsupported_model() {
    if !should_run_integration_tests() {
        println!("⏭️  Skipping integration test: OPENAI_API_KEY not set");
        return;
    }

    let config = ExecutorConfig::from_env();
    let service =
        ExecutorService::from_config(config).expect("Failed to create ExecutorService");

    let plan = create_test_route_plan("openai", "gpt-5-ultra");
    let payload = create_test_payload("Test");

    // Try to execute with unsupported model
    let result = service.execute(&plan, &payload).await;

    assert!(result.is_err(), "Should fail with unsupported model");
    println!("✅ Unsupported model test passed (correctly rejected)");
}

#[tokio::test]
async fn test_openai_api_cost_calculation() {
    if !should_run_integration_tests() {
        println!("⏭️  Skipping integration test: OPENAI_API_KEY not set");
        return;
    }

    let config = ExecutorConfig::from_env();
    let service =
        ExecutorService::from_config(config).expect("Failed to create ExecutorService");

    let plan = create_test_route_plan("openai", "gpt-3.5-turbo");
    let payload = json!({
        "messages": [
            {"role": "user", "content": "Count from 1 to 5."}
        ],
        "temperature": 0.5,
        "max_tokens": 50
    });

    let result = service
        .execute(&plan, &payload)
        .await
        .expect("API call should succeed");

    println!("✅ Cost calculation test:");
    println!("   Prompt tokens: {}", result.prompt_tokens);
    println!("   Completion tokens: {}", result.completion_tokens);
    println!("   Total cost: ${:.6}", result.total_cost);

    // Verify cost is calculated (should be > 0 for gpt-3.5-turbo)
    assert!(
        result.total_cost > 0.0,
        "Cost should be calculated and positive"
    );

    // Verify cost is reasonable (gpt-3.5-turbo is cheap, should be < $0.01 for this test)
    assert!(
        result.total_cost < 0.01,
        "Cost should be reasonable for small request"
    );
}
