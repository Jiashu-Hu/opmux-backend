use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;

/// Errors specific to the Health feature.
///
/// This enum represents all possible error conditions that can occur during
/// health check operations. Each variant corresponds to a specific business
/// operation that failed, providing rich context for debugging and monitoring.
///
/// Following the error handling standard, these errors are modeled by business
/// operation rather than technical cause, making them more actionable for
/// developers and operators.
///
/// # Error Categories
///
/// All health errors are server-side (5xx) errors since health checks are
/// internal system operations that don't typically have client-side failures.
///
/// # HTTP Status Mapping
///
/// All variants map to `503 Service Unavailable` to indicate the service is
/// temporarily unable to handle requests due to health check failures.
#[derive(Debug, thiserror::Error)]
pub enum HealthError {
    /// System status check operation failed.
    ///
    /// This error occurs when the basic system health check cannot be completed,
    /// typically due to critical system component failures.
    ///
    /// # Common Causes
    /// - Database connectivity issues
    /// - File system access problems
    /// - Critical service dependencies unavailable
    #[error("System status check failed")]
    SystemStatusCheckFailed,

    /// Dependency validation operation failed.
    ///
    /// This error occurs when checking external service dependencies fails,
    /// indicating that required external services are unavailable.
    ///
    /// # Common Causes
    /// - External API endpoints unreachable
    /// - Authentication failures with dependencies
    /// - Network connectivity issues
    #[error("Dependency validation failed")]
    DependencyValidationFailed,

    /// Resource monitoring operation failed.
    ///
    /// This error occurs when system resource monitoring cannot be completed,
    /// typically due to insufficient system resources or monitoring failures.
    ///
    /// # Common Causes
    /// - Memory usage critically high
    /// - Disk space exhausted
    /// - CPU usage at maximum capacity
    /// - Monitoring system failures
    #[error("Resource monitoring failed")]
    ResourceMonitoringFailed,

    /// Health aggregation operation failed.
    ///
    /// This error occurs when combining multiple health check results fails,
    /// typically due to inconsistent or conflicting health data.
    ///
    /// # Common Causes
    /// - Conflicting health check results
    /// - Health data corruption
    /// - Aggregation logic errors
    #[error("Health aggregation failed")]
    HealthAggregationFailed,
}

impl IntoResponse for HealthError {
    /// Converts health errors into HTTP responses.
    ///
    /// This implementation provides consistent HTTP response formatting for all
    /// health check errors. All health errors are mapped to `503 Service Unavailable`
    /// to indicate temporary service unavailability.
    ///
    /// # Response Format
    ///
    /// ```json
    /// {
    ///   "error": "Health check failed - service temporarily unavailable."
    /// }
    /// ```
    ///
    /// # HTTP Status Codes
    ///
    /// - `503 Service Unavailable` - All health check failures
    ///
    /// This status code is appropriate because health check failures indicate
    /// the service is temporarily unable to handle requests, but may recover.
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
