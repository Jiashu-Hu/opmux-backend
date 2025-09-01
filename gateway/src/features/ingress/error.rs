use axum::{http::StatusCode, response::{IntoResponse, Response, Json}};
use serde_json::json;

/// Errors specific to the Ingress feature.
/// Each variant corresponds to a failed business operation.
#[derive(Debug, thiserror::Error)]
pub enum IngressError {
    // Client-side errors (4xx)
    #[error("Invalid request format: {0}")]
    InvalidRequest(String),
    
    #[error("Missing required authentication")]
    AuthenticationRequired,
    
    #[error("Insufficient permissions for this operation")]
    AuthorizationFailed,
    
    // Server-side business operation errors (5xx)
    #[error("Failed to retrieve conversation context")]
    ContextRetrievalFailed,
    
    #[error("Request orchestration failed")]
    RequestOrchestrationFailed,
    
    #[error("Response aggregation failed")]
    ResponseAggregationFailed,
    
    #[error("Failed to update conversation context")]
    ContextUpdateFailed,
}

impl IntoResponse for IngressError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            // Client errors are mapped to 4xx status codes.
            Self::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::AuthenticationRequired => (
                StatusCode::UNAUTHORIZED,
                "Authentication is required to access this resource.".to_string(),
            ),
            Self::AuthorizationFailed => (
                StatusCode::FORBIDDEN,
                "You do not have permission to perform this operation.".to_string(),
            ),
            
            // Server-side business logic failures are mapped to 5xx status codes.
            // Return a generic message to the user for security.
            Self::ContextRetrievalFailed 
            | Self::RequestOrchestrationFailed 
            | Self::ResponseAggregationFailed 
            | Self::ContextUpdateFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal error occurred while processing your request.".to_string(),
            ),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
