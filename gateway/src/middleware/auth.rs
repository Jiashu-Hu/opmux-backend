// Authentication middleware - protects endpoints with API key validation
// Now uses the 3-layer architecture from features/auth

use crate::features::auth::{get_auth_config, AuthContext, AuthService};
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

/// Authentication middleware function
/// Validates X-API-Key header and injects AuthContext into request
/// Supports development mode bypass via AUTH_DEVELOPMENT_MODE environment variable
pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let config = get_auth_config();

    // Check if development mode is enabled
    if config.is_development_mode() {
        // Development mode: bypass authentication and inject mock context
        let auth_service = AuthService::new();
        let dev_context = auth_service.create_dev_context();
        request.extensions_mut().insert(dev_context);
        return Ok(next.run(request).await);
    }

    // Production mode: require API key authentication
    let headers = request.headers();

    // Get API key from X-API-Key header
    let api_key = match extract_api_key(headers) {
        Some(key) => key,
        None => {
            tracing::warn!("Authentication failed: Missing X-API-Key header");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Validate API key using auth service
    let auth_context = match validate_api_key(&api_key).await {
        Some(context) => context,
        None => {
            tracing::warn!("Authentication failed: Invalid API key: {}", api_key);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Inject AuthContext into request extensions
    request.extensions_mut().insert(auth_context);

    // Continue to next handler
    Ok(next.run(request).await)
}

/// Extract API key from X-API-Key header
fn extract_api_key(headers: &HeaderMap) -> Option<String> {
    headers
        .get("X-API-Key")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}

/// Validate API key using the auth service
/// This replaces the hardcoded validation logic
async fn validate_api_key(api_key: &str) -> Option<AuthContext> {
    let auth_service = AuthService::new();
    auth_service.validate_api_key(api_key).await
}
