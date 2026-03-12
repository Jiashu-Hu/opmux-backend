//! Ingress module for AI routing request processing.
//!
//! Handles incoming AI routing requests and coordinates AI response generation
//! through microservice orchestration. Follows 3-layer architecture with
//! structured error handling.
//!
//! # Request Flow
//!
//! 1. **Handler** - Validates HTTP requests and extracts data
//! 2. **Service** - Orchestrates business logic and microservice calls
//! 3. **Repository** - Manages gRPC communication with AI services
//!
//! # Usage
//!
//! ```bash
//! curl -X POST http://localhost:3000/api/v1/route \
//!   -H "Content-Type: application/json" \
//!   -d '{"prompt": "Hello!", "metadata": {}}'
//! ```
//!
//! The service normalizes `prompt` into `messages` before execution.

/// Error handling for ingress operations.
pub mod error;
/// Handler Layer - HTTP request/response processing.
pub mod handler;
/// Repository Layer - gRPC client management & mocks.
pub mod repository;
/// Service Layer - Business logic and orchestration.
pub mod service;

/// Constants and hardcoded values.
pub mod constants;
/// Mock data for development and testing.
pub mod mockdata;

#[cfg(test)]
mod handler_tests;
#[cfg(test)]
mod service_tests;

// Re-export the handler for easy access
pub use handler::ingress_handler;
