### **Standard for Designing and Implementing Layered Error Handling**

**Your Goal:** You are an expert Rust developer. Your task is to design and implement error handling
for a new feature module within our application. You must follow this standard precisely.

### 1. Core Philosophy: Model by Business Operation

This is the most important principle. Our errors are modeled to describe the specific **business
operation** or **process step** that failed. This provides rich, actionable context for debugging.

We **DO NOT** model errors based on their low-level _source_ or _technical cause_.

- **CORRECT ✅**: `enum IngressError { ContextRetrievalFailed, ... }` _(This tells us **what** we
  were trying to do.)_

- **INCORRECT ❌**: `enum IngressError { MemoryServiceError, GrpcTimeout, ... }` _(This only tells
  us **which** dependency or technical component failed, losing valuable business context.)_

### 2. The Two-Layer Structure

Our error handling has two distinct layers: the **Feature Layer** and the **Core Layer**.

- **Feature-Level Errors**: Each feature (e.g., `ingress`, `auth`) has its own error enum that is
  specific to its domain.
  - Location: `src/features/your_feature/error.rs`
  - Example: `pub enum YourFeatureError { ... }`

- **Top-Level `AppError`**: A single enum at the application's core that aggregates all
  feature-level errors.
  - Location: `src/core/error.rs`
  - Purpose: To act as a unified error type for the entire application, using `#[from]` to delegate
    to the specific feature errors.

### 3. Implementation Workflow

When adding error handling to a new feature named `your_feature`, follow these steps:

#### Step 1: Analyze the Feature's Business Logic

Before writing code, identify the distinct, high-level operations the feature's service performs.
For example, an "ingress" service might: 1. Validate input, 2. Retrieve data, 3. Process data, 4.
Save data. These operations will become your error variants.

#### Step 2: Create the Feature-Level Error File

Create a new file at `src/features/your_feature/error.rs`. Use the template below.

**Template: `src/features/your_feature/error.rs`**

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response, Json}};
use serde_json::json;

/// Errors specific to "Your Feature".
/// Each variant corresponds to a failed business operation.
#[derive(Debug, thiserror::Error)]
pub enum YourFeatureError {
    #[error("Invalid input provided: {0}")]
    InvalidInput(String),

    #[error("Operation one failed.")]
    OperationOneFailed,

    #[error("Operation two failed.")]
    OperationTwoFailed,
}

impl IntoResponse for YourFeatureError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            // Client errors are mapped to 4xx status codes.
            Self::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),

            // Server-side business logic failures are mapped to 5xx status codes.
            // Return a generic message to the user for security.
            Self::OperationOneFailed | Self::OperationTwoFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal error occurred.".to_string(),
            ),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
```

#### Step 3: Update the Top-Level `AppError`

Finally, integrate your new feature error into the application's core error system.

**Template: `src/core/error.rs`**

```rust
use axum::response::{IntoResponse, Response};
// Import your new feature's error module
use crate::features::your_feature;

/// The single, top-level error type for the entire application.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // Add your new feature error here using #[from].
    // The `transparent` attribute passes the Display implementation up.
    #[error(transparent)]
    YourFeature(#[from] your_feature::error::YourFeatureError),

    // --- Add other features below as they are created ---
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // In a real app, use a structured logger like tracing.
        tracing::error!("An error occurred: {:?}", self);

        // Delegate response generation to the underlying feature error.
        match self {
            AppError::YourFeature(e) => e.into_response(),
        }
    }
}
```
