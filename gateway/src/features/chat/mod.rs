// Chat feature module - Core business logic
// Implements 3-layer architecture: Handler -> Service -> Repository

pub mod handler;    // Chat endpoint handlers (API endpoints)
pub mod service;    // Request orchestration and business logic
pub mod repository; // gRPC client management and data access
pub mod models;     // Request/response models and data structures
