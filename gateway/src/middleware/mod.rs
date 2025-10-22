// HTTP middleware modules

pub mod auth;
pub mod correlation_id;

// Re-export middleware functions for convenience
pub use correlation_id::correlation_id_middleware;
