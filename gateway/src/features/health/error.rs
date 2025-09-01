use axum::{http::StatusCode, response::{IntoResponse, Response, Json}};
use serde_json::json;

/// Errors specific to the Health feature.
/// Each variant corresponds to a failed business operation.
#[derive(Debug, thiserror::Error)]
pub enum HealthError {
    // Server-side business operation errors (5xx)
    // Health checks typically don't have client errors (4xx)
    
    #[error("System status check failed")]
    SystemStatusCheckFailed,
    
    #[error("Dependency validation failed")]
    DependencyValidationFailed,
    
    #[error("Resource monitoring failed")]
    ResourceMonitoringFailed,
    
    #[error("Health aggregation failed")]
    HealthAggregationFailed,
}

impl IntoResponse for HealthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            // Health check failures should return 503 Service Unavailable
            // This indicates the service is temporarily unable to handle requests
            Self::SystemStatusCheckFailed 
            | Self::DependencyValidationFailed 
            | Self::ResourceMonitoringFailed 
            | Self::HealthAggregationFailed => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Health check failed - service temporarily unavailable.".to_string(),
            ),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
