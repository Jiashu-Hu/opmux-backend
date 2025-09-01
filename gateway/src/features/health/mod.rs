// Health check module following 3-layer architecture

pub mod error; // Error handling for health operations
pub mod handler; // Handler Layer - HTTP request/response processing
pub mod repository; // Repository Layer - Data access & mocks
pub mod service; // Service Layer - Business logic abstraction

// Re-export the handler for easy access
pub use handler::health_handler;
