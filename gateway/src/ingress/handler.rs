// Handler Layer - HTTP request/response processing

use super::service::{IngressRequest, IngressResponse, IngressService};
use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson};

// Handler function - only handles HTTP requests, validation, and response formatting
pub async fn ingress_handler(
    Json(request): Json<IngressRequest>,
) -> Result<ResponseJson<IngressResponse>, StatusCode> {
    // Basic request validation
    if request.prompt.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Create service instance
    let service = IngressService::new();

    // For now, use a mock user_id - in future this will come from JWT authentication
    let user_id = "mock_user_123".to_string();

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
