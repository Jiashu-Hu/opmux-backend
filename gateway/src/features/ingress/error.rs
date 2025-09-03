use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;

/// Errors for ingress AI routing processing operations.
///
/// Each variant represents a specific business operation failure,
/// providing clear context for debugging and monitoring.
#[derive(Debug, thiserror::Error)]
pub enum IngressError {
    /// Invalid client request (400 Bad Request).
    #[error("Invalid request format: {0}")]
    InvalidRequest(String),

    /// Missing authentication (401 Unauthorized).
    #[error("Missing required authentication")]
    AuthenticationRequired,

    /// Insufficient permissions (403 Forbidden).
    #[error("Insufficient permissions for this operation")]
    AuthorizationFailed,

    /// Context retrieval from Memory Service failed (500 Internal Server Error).
    #[error("Failed to retrieve conversation context")]
    ContextRetrievalFailed,

    /// AI request orchestration failed (500 Internal Server Error).
    #[error("Request orchestration failed")]
    RequestOrchestrationFailed,

    /// Response aggregation failed (500 Internal Server Error).
    #[error("Response aggregation failed")]
    ResponseAggregationFailed,

    /// Context update to Memory Service failed (500 Internal Server Error).
    #[error("Failed to update conversation context")]
    ContextUpdateFailed,
}

impl IntoResponse for IngressError {
    /// Converts ingress errors into HTTP JSON responses with appropriate status codes.
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
