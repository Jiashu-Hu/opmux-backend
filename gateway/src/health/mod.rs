// Health check module following 3-layer architecture

pub mod handler;    // Handler Layer - HTTP request/response processing
pub mod service;    // Service Layer - Business logic abstraction  
pub mod repository; // Repository Layer - Data access & mocks

// Re-export the handler for easy access
pub use handler::health_handler;
