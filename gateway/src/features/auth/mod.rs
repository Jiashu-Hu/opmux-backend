//! Authentication Feature Module
//!
//! Provides API key authentication functionality following 3-layer architecture:
//! - Handler Layer: HTTP endpoints for API key management (future)
//! - Service Layer: Authentication business logic
//! - Repository Layer: Data access with mock implementation

// Export public interfaces
pub use config::{get_auth_config, AuthConfig};
pub use error::AuthError;
pub use models::*;
pub use service::AuthService;

// Internal modules
pub mod config;
pub mod error;
mod mockdata;
mod models;
mod repository;
mod service;

// Future: handler module for API key management endpoints
// pub mod handler;
