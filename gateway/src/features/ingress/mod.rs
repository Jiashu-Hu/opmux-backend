// Ingress module following 3-layer architecture
// Handles incoming requests and coordinates microservices

pub mod handler; // Handler Layer - HTTP request/response processing
pub mod repository;
pub mod service; // Service Layer - Business logic and orchestration // Repository Layer - gRPC client management & mocks

// Re-export the handler for easy access
pub use handler::ingress_handler;
