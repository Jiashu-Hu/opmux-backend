# Rust Documentation Template

A simple, consistent template for documenting Rust code following these principles:
- **Simple** - Easy to read and understand
- **Brief** - No unnecessary verbosity  
- **Informative** - Contains all important messages
- **Consistent** - Uniform structure and style

## Module Documentation (`mod.rs`)

Use `//!` for module-level documentation:

```rust
//! Feature module for [specific functionality].
//! 
//! [Brief description of what this module does and its role in the system.]
//! 
//! # Architecture
//! 
//! - **Handler** - HTTP request/response processing
//! - **Service** - Business logic and orchestration
//! - **Repository** - Data access and external service communication
//! - **Error** - Structured error handling
//! 
//! # Usage
//! 
//! ```bash
//! curl -X POST http://localhost:3000/endpoint \
//!   -H "Content-Type: application/json" \
//!   -d '{"field": "value"}'
//! ```

/// Error handling for [feature] operations.
pub mod error;
/// Handler Layer - HTTP request/response processing.
pub mod handler;
/// Repository Layer - Data access & external services.
pub mod repository;
/// Service Layer - Business logic and orchestration.
pub mod service;
```

## Struct Documentation

### Simple Structs
```rust
/// [Brief description of what this struct represents].
#[derive(Debug, Clone)]
pub struct DataStruct {
    /// [Field purpose and meaning].
    pub field_name: String,
    /// [Field purpose with constraints/format if relevant].
    pub another_field: u64,
}
```

### Service/Repository Structs
```rust
/// Service for [feature] business logic and orchestration.
/// 
/// [Brief description of responsibilities and role in architecture.]
pub struct FeatureService {
    repository: FeatureRepository,
}
```

## Function Documentation

### Simple Functions
```rust
/// [Brief description of what the function does].
pub fn simple_function() -> String {
    // implementation
}
```

### Functions with Parameters
```rust
/// [Brief description of what the function does].
/// 
/// # Parameters
/// - `param1` - [Purpose and meaning]
/// - `param2` - [Purpose and constraints]
/// 
/// # Returns
/// [What the function returns and its meaning]
pub fn function_with_params(param1: &str, param2: u64) -> Result<String, Error> {
    // implementation
}
```

### Complex Functions (Business Logic)
```rust
/// [Brief description of the main purpose].
/// 
/// # Flow
/// 1. [First step description]
/// 2. [Second step description]
/// 3. [Third step description]
/// 4. [Final step description]
/// 
/// # Parameters
/// - `param1` - [Purpose and meaning]
/// - `param2` - [Purpose and constraints]
/// 
/// # Returns
/// [What the function returns with business context]
pub async fn complex_function(
    param1: RequestType,
    param2: String,
) -> Result<ResponseType, FeatureError> {
    // implementation
}
```

## Error Documentation

### Error Enum
```rust
/// Errors for [feature] operations.
/// 
/// Each variant represents a specific business operation failure,
/// providing clear context for debugging and monitoring.
#[derive(Debug, thiserror::Error)]
pub enum FeatureError {
    /// [Client error description] ([HTTP status]).
    #[error("[Error message]")]
    ClientError(String),
    
    /// [Server error description] ([HTTP status]).
    #[error("[Error message]")]
    ServerError,
}
```

### Error Implementation
```rust
impl IntoResponse for FeatureError {
    /// Converts [feature] errors into HTTP JSON responses with appropriate status codes.
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::ClientError(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::ServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal error occurred.".to_string(),
            ),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
```

## Handler Documentation

```rust
/// HTTP handler for [endpoint purpose].
/// 
/// # Flow
/// 1. [Validation step]
/// 2. [Processing step]
/// 3. [Response step]
/// 
/// # Parameters
/// - `request` - [Request structure and content]
/// 
/// # Returns
/// [Response structure and content]
pub async fn feature_handler(
    Json(request): Json<RequestType>,
) -> Result<Json<ResponseType>, FeatureError> {
    // implementation
}
```

## Constants Documentation

```rust
/// [Group description].
pub const CONSTANT_NAME: &str = "value";

/// [Group description with units/format if applicable].
pub const TIMEOUT_MS: u64 = 5000;
```

## Documentation Checklist

### ✅ Required Elements
- [ ] Brief, clear description of purpose
- [ ] Parameter documentation for all inputs
- [ ] Return value documentation
- [ ] Error conditions (if applicable)
- [ ] Business context (why it exists)

### ✅ Optional Elements (when helpful)
- [ ] Flow steps for complex logic
- [ ] Usage examples
- [ ] Future enhancement notes
- [ ] HTTP status codes for errors
- [ ] Constraints or validation rules

### ✅ Style Guidelines
- [ ] Start with action verb for functions ("Processes", "Retrieves", "Updates")
- [ ] Use present tense
- [ ] Keep descriptions under 2 lines when possible
- [ ] Use consistent section headers (`# Flow`, `# Parameters`, `# Returns`)
- [ ] Include business context, not just technical details
- [ ] Mention HTTP status codes for web handlers
- [ ] Note future integration plans where relevant

### ✅ Common Patterns

**For Services:**
```rust
/// Service for [feature] [main responsibility].
/// 
/// [Brief architecture role description.]
```

**For Repositories:**
```rust
/// Repository for [feature] data access and [external system] communication.
/// 
/// [Brief description of what external systems it manages.]
```

**For Handlers:**
```rust
/// HTTP handler for [endpoint] endpoint.
/// 
/// [Brief description of request processing and response.]
```

**For Errors:**
```rust
/// Errors for [feature] [operation type] operations.
/// 
/// Each variant represents a specific business operation failure.
```

This template ensures documentation is simple, brief, informative, and consistent across all Rust code.
