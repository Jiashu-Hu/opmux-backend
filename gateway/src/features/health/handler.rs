// Handler Layer - HTTP request/response processing

use super::{error::HealthError, service::HealthService};
use axum::response::Json;

// Handler function - only handles HTTP requests, validation, and response formatting
pub async fn health_handler() -> Result<Json<super::service::HealthResponse>, HealthError> {
    let service = HealthService::new();

    match service.check_health().await {
        Ok(health_status) => Ok(Json(health_status)),
        Err(error) => {
            // Error logging is now handled by AppError in core/error.rs
            Err(error)
        }
    }
}
