//! Executor Layer - LLM API execution and vendor integration.
//!
//! This module is responsible for executing actual LLM API calls based on routing decisions
//! from RouterService. It handles vendor SDK integration, parameter extraction, and response
//! processing.
//!
//! # Architecture
//!
//! - **Service Layer** (`service.rs`) - Orchestrates LLM execution workflow
//! - **Vendor Layer** (`vendors/`) - Vendor-specific API integrations (OpenAI, Anthropic, etc.)
//! - **Models** (`models.rs`) - Data structures for execution requests and results
//! - **Error Handling** (`error.rs`) - Executor-specific error types
//! - **Configuration** (`config.rs`) - Executor configuration (API keys, timeouts)
//!
//! # Responsibilities
//!
//! - Execute LLM API calls based on RoutePlan (vendor_id, model_id)
//! - Extract execution parameters from original_payload (temperature, max_tokens, etc.)
//! - Integrate multiple LLM vendor SDKs (OpenAI, Anthropic, Cohere, etc.)
//! - Handle HTTP calls, retries, timeouts
//! - Calculate actual token usage and costs
//! - Support streaming responses (future)
//!
//! # NOT Responsible For
//!
//! - Routing decisions (RouterService's responsibility)
//! - Prompt rewriting (RewriteService's responsibility)
//! - Context management (MemoryService's responsibility)
//!
//! # Usage
//!
//! ```rust,no_run
//! use gateway::executor::{ExecutorService, ExecutionRequest};
//! use gateway::features::ingress::repository::RoutePlan;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let executor = ExecutorService::new();
//!
//! let plan = RoutePlan {
//!     vendor_id: "openai".to_string(),
//!     model_id: "gpt-4".to_string(),
//!     fallback_plans: vec![],
//! };
//!
//! let payload = serde_json::json!({
//!     "messages": [{"role": "user", "content": "Hello!"}],
//!     "temperature": 0.7,
//! });
//!
//! let result = executor.execute(&plan, &payload).await?;
//! println!("AI Response: {}", result.content);
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod error;
pub mod models;
pub mod service;
pub mod vendors;

// Re-export commonly used types
pub use error::ExecutorError;
pub use models::{ExecutionParams, ExecutionRequest, ExecutionResult};
pub use service::ExecutorService;

