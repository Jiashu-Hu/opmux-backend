# Rust Documentation Guide

## Core Principles

**Simple** - Easy to read and understand  
**Brief** - No unnecessary verbosity  
**Informative** - Contains all important messages  
**Consistent** - Uniform structure and style

## Documentation Structure

### 1. Module Documentation (`mod.rs`)

````rust
//! Module purpose and overview.
//!
//! Brief description of what this module does and its role in the system.
//!
//! # Architecture
//!
//! - **Layer 1** - Purpose and responsibility
//! - **Layer 2** - Purpose and responsibility
//! - **Layer 3** - Purpose and responsibility
//!
//! # Usage
//!
//! ```bash
//! # Simple usage example
//! curl -X POST http://localhost:3000/endpoint
//! ```

/// Brief description of submodule purpose.
pub mod submodule;
````

### 2. Struct Documentation

```rust
/// Brief struct purpose.
///
/// Longer description if needed, explaining role in the system.
/// Future plans or important notes.
pub struct ServiceName {
    /// Field purpose and what it contains.
    field_name: Type,
}
```

### 3. Function Documentation

```rust
/// Brief function purpose (what it does).
///
/// # Flow (for complex functions)
/// 1. Step one description
/// 2. Step two description
/// 3. Step three description
///
/// # Parameters
/// - `param1` - Purpose and what it represents
/// - `param2` - Purpose and constraints
///
/// # Returns
/// What the function returns and its meaning
///
/// # Errors (if applicable)
/// When errors occur and what they mean
pub async fn function_name(param1: Type, param2: Type) -> Result<ReturnType, ErrorType> {
```

### 4. Error Documentation

```rust
/// Errors for [feature] operations.
///
/// Each variant represents a specific business operation failure,
/// providing clear context for debugging and monitoring.
#[derive(Debug, thiserror::Error)]
pub enum FeatureError {
    /// Brief error description (HTTP status if applicable).
    #[error("Error message")]
    ErrorVariant,
}

impl IntoResponse for FeatureError {
    /// Converts errors into HTTP JSON responses with appropriate status codes.
    fn into_response(self) -> Response {
```

## Documentation Sections

### Required Sections

- **Brief description** - One line explaining purpose
- **Parameters** - For functions with inputs
- **Returns** - For functions with outputs

### Optional Sections (use when helpful)

- **Flow** - For complex multi-step processes
- **Errors** - For functions that can fail
- **Examples** - For complex usage patterns
- **Future** - For planned enhancements

## Writing Style

### Do ✅

- Start with action verbs ("Processes", "Retrieves", "Updates")
- Use present tense ("Returns user data")
- Be specific ("User identifier" not "ID")
- Include business context ("for context management")
- Mention HTTP status codes for errors
- Note future plans briefly

### Don't ❌

- Use unnecessary words ("This function", "This method")
- Repeat information from function signature
- Write long paragraphs
- Include implementation details
- Use technical jargon without explanation

## Examples

### Good Documentation ✅

```rust
/// Processes chat request through the complete AI pipeline.
///
/// # Flow
/// 1. Retrieves conversation context from Memory Service
/// 2. Processes prompt (applies rewrite if requested)
/// 3. Routes to AI services via Router Service
/// 4. Updates conversation context with new exchange
///
/// # Parameters
/// - `request` - Chat request with prompt and metadata
/// - `user_id` - User identifier for context management
///
/// # Returns
/// Complete AI response with metadata (cost, model, processing time)
pub async fn process_request(&self, request: IngressRequest, user_id: String) -> Result<IngressResponse, IngressError>
```

### Poor Documentation ❌

```rust
/// This function processes a request
///
/// This method takes an IngressRequest and a String representing the user ID,
/// then it goes through several steps to process the request by calling various
/// services and then returns an IngressResponse or an error if something goes wrong
pub async fn process_request(&self, request: IngressRequest, user_id: String) -> Result<IngressResponse, IngressError>
```

## Quick Checklist

- [ ] Brief, clear purpose statement
- [ ] Parameters documented with purpose
- [ ] Return value explained
- [ ] Complex logic broken into numbered steps
- [ ] Business context included
- [ ] HTTP status codes noted for errors
- [ ] Future plans mentioned if relevant
- [ ] No unnecessary verbosity
- [ ] Consistent with existing documentation
