// Handler Layer - HTTP request/response processing

use super::{
    error::IngressError,
    mockdata::MockDataProvider,
    service::{IngressRequest, IngressResponse, IngressService},
};
use axum::{extract::Json, response::Json as ResponseJson};

// Handler function - only handles HTTP requests, validation, and response formatting
// TODO: In future, this will extract user_id from JWT token in Authorization header
pub async fn ingress_handler(
    Json(request): Json<IngressRequest>,
) -> Result<ResponseJson<IngressResponse>, IngressError> {
    // Basic request validation
    if request.prompt.trim().is_empty() {
        return Err(IngressError::InvalidRequest("Prompt cannot be empty".to_string()));
    }

    // Create service instance
    let service = IngressService::new();

    // Use mock user_id from mockdata - in future this will be extracted from JWT token
    let user_id = MockDataProvider::get_mock_user_id();

    // Process the request through service layer
    match service.process_request(request, user_id).await {
        Ok(response) => Ok(ResponseJson(response)),
        Err(error) => {
            // Error logging is now handled by AppError in core/error.rs
            Err(error)
        }
    }
}
