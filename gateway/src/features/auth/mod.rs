// Authentication and authorization feature module
// Implements 3-layer architecture: Handler -> Service -> Repository

pub mod handler;    // Authentication handlers (API endpoints)
pub mod service;    // JWT validation and business logic
pub mod repository; // Supabase integration and data access
pub mod models;     // Auth data structures and models
