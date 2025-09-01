// Health check module following 3-layer architecture

pub mod handler; // Handler Layer - HTTP request/response processing
pub mod repository;
pub mod service; // Service Layer - Business logic abstraction // Repository Layer - Data access & mocks

// Re-export the handler for easy access
pub use handler::health_handler;
