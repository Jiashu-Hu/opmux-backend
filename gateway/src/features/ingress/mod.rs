// Ingress module following 3-layer architecture
// Handles incoming requests and coordinates microservices

pub mod error; // Error handling for ingress operations
pub mod handler; // Handler Layer - HTTP request/response processing
pub mod repository; // Repository Layer - gRPC client management & mocks
pub mod service; // Service Layer - Business logic and orchestration

// Configuration and data modules
pub mod constants; // Constants and hardcoded values
pub mod mockdata; // Mock data for development and testing

// Re-export the handler for easy access
pub use handler::ingress_handler;
