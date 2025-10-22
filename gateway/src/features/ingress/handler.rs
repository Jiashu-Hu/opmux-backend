// Handler Layer - HTTP request/response processing

use super::{
    error::IngressError,
    service::{IngressRequest, IngressResponse, IngressService},
};
use crate::{core::correlation::RequestContext, features::auth::AuthContext, AppState};
use axum::{
    extract::{Extension, Json, State},
    response::Json as ResponseJson,
};

/// HTTP handler for AI routing ingress endpoint.
///
/// This is the ROOT SPAN for request tracing. All child spans automatically
/// inherit `request_id` and `client_correlation_id` from this span.
///
/// # Flow
/// 1. Validates request (non-empty prompt)
/// 2. Extracts user_id from authentication context
/// 3. Processes request through service layer
/// 4. Returns JSON response or error
///
/// # Parameters
/// - `state` - Application state with shared services (injected via Axum state)
/// - `request_context` - Request correlation context (injected by correlation_id_middleware)
/// - `auth_context` - Authentication context (injected by auth middleware)
/// - `request` - JSON AI routing request with prompt and metadata
///
/// # Returns
/// JSON response with AI content, model info, cost, and timing
#[tracing::instrument(
    skip(state, request_context, auth_context, request),
    fields(
        request_id = %request_context.request_id,
        client_correlation_id = ?request_context.client_correlation_id,
        user_id = %auth_context.client_id,
        endpoint = "/api/v1/route",
        prompt_length = request.prompt.len(),
    )
)]
pub async fn ingress_handler(
    State(state): State<AppState>,
    Extension(request_context): Extension<RequestContext>,
    auth_context: AuthContext,
    Json(request): Json<IngressRequest>,
) -> Result<ResponseJson<IngressResponse>, IngressError> {
    tracing::info!("Incoming AI routing request");

    // Basic request validation
    if request.prompt.trim().is_empty() {
        tracing::warn!("Request validation failed: empty prompt");
        return Err(IngressError::InvalidRequest(
            "Prompt cannot be empty".to_string(),
        ));
    }

    tracing::debug!("Request validation passed");

    // Create service instance with ExecutorService dependency
    let service = IngressService::new(state.executor_service);

    // Use client_id from authentication context
    let user_id = auth_context.client_id;

    // Process the request through service layer
    // Pass request_context for gRPC metadata construction
    match service
        .process_request(request, user_id, &request_context)
        .await
    {
        Ok(response) => {
            tracing::info!(
                model = %response.model_used,
                cost = response.cost,
                processing_time_ms = response.processing_time_ms,
                "Request processing completed successfully"
            );
            Ok(ResponseJson(response))
        }
        Err(error) => {
            tracing::error!(error = ?error, "Request processing failed");
            Err(error)
        }
    }
}
