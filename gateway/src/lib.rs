//! Gateway Service Library
//!
//! AI API Router Gateway Microservice implementing 3-layer architecture:
//! - Handler Layer: HTTP request/response processing
//! - Service Layer: Business logic and orchestration
//! - Repository Layer: Data access and gRPC client management

// Core modules - Reusable primitives
pub mod core;

// Configuration management
pub mod config;

// Business feature modules
pub mod features;

// HTTP middleware
pub mod middleware;

// gRPC client abstractions
pub mod grpc;
