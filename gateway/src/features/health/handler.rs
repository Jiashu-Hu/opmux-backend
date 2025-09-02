// Handler Layer - HTTP request/response processing

use super::{error::HealthError, service::HealthService};
use axum::response::Json;

/// HTTP handler for health check endpoints.
///
/// This handler provides a RESTful health check endpoint that returns the current
/// system health status in JSON format. It follows the handler layer pattern by
/// focusing solely on HTTP request/response processing while delegating business
/// logic to the service layer.
///
/// # HTTP Response
///
/// ## Success Response (200 OK)
/// ```json
/// {
///   "status": "healthy",
///   "timestamp": "2025-09-01T16:53:30.625665+00:00"
/// }
/// ```
///
/// ## Error Response (503 Service Unavailable)
/// ```json
/// {
///   "error": "Health check failed - service temporarily unavailable."
/// }
/// ```
///
/// # Usage
///
/// This handler is typically mounted at `/health` and used by:
/// - Load balancers for health checks
/// - Monitoring systems for service availability
/// - Kubernetes liveness/readiness probes
/// - Manual health verification
///
/// # Examples
///
/// ```bash
/// curl http://localhost:3000/health
/// ```
pub async fn health_handler() -> Result<Json<super::service::HealthResponse>, HealthError>
{
    let service = HealthService::new();

    match service.check_health().await {
        Ok(health_status) => Ok(Json(health_status)),
        Err(error) => {
            // Error logging is now handled by AppError in core/error.rs
            Err(error)
        }
    }
}
