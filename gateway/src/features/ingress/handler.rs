// Handler Layer - HTTP request/response processing

use super::{
    constants::ERROR_EMPTY_PROMPT,
    mockdata::MockDataProvider,
    service::{IngressRequest, IngressResponse, IngressService},
};
use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson};

// Handler function - only handles HTTP requests, validation, and response formatting
// TODO: In future, this will extract user_id from JWT token in Authorization header
pub async fn ingress_handler(
    Json(request): Json<IngressRequest>,
) -> Result<ResponseJson<IngressResponse>, StatusCode> {
    // Basic request validation using constants
    if request.prompt.trim().is_empty() {
        // Log validation error (in future, use proper logging)
        eprintln!("Validation error: {}", ERROR_EMPTY_PROMPT);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Create service instance
    let service = IngressService::new();

    // Use mock user_id from mockdata - in future this will be extracted from JWT token
    let user_id = MockDataProvider::get_mock_user_id();

    // Process the request through service layer
    match service.process_request(request, user_id).await {
        Ok(response) => Ok(ResponseJson(response)),
        Err(error) => {
            // Log error (in future, use proper logging)
            eprintln!("Ingress processing error: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
