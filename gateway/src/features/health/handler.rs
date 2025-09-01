// Handler Layer - HTTP request/response processing

use super::service::HealthService;
use axum::{http::StatusCode, response::Json};

// Handler function - only handles HTTP requests, validation, and response formatting
pub async fn health_handler() -> Result<Json<super::service::HealthResponse>, StatusCode> {
    let service = HealthService::new();

    match service.check_health().await {
        Ok(health_status) => Ok(Json(health_status)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
