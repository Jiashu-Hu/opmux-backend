// Handler Layer - HTTP request/response processing

use super::{
    error::IngressError,
    service::{IngressRequest, IngressResponse, IngressService},
};
use crate::features::auth::AuthContext;
use axum::{extract::Json, response::Json as ResponseJson};

/// HTTP handler for AI routing ingress endpoint.
///
/// # Flow
/// 1. Validates request (non-empty prompt)
/// 2. Extracts user_id (currently mock, future: from JWT)
/// 3. Processes request through service layer
/// 4. Returns JSON response or error
///
/// # Parameters
/// - `request` - JSON AI routing request with prompt and metadata
///
/// # Returns
/// JSON response with AI content, model info, cost, and timing
///
/// AuthContext is injected by authentication middleware.
pub async fn ingress_handler(
    auth_context: AuthContext,
    Json(request): Json<IngressRequest>,
) -> Result<ResponseJson<IngressResponse>, IngressError> {
    // Basic request validation
    if request.prompt.trim().is_empty() {
        return Err(IngressError::InvalidRequest(
            "Prompt cannot be empty".to_string(),
        ));
    }

    // Create service instance
    let service = IngressService::new();

    // Use client_id from authentication context
    let user_id = auth_context.client_id;

    // Process the request through service layer
    match service.process_request(request, user_id).await {
        Ok(response) => Ok(ResponseJson(response)),
        Err(error) => {
            // Error logging is now handled by AppError in core/error.rs
            Err(error)
        }
    }
}
