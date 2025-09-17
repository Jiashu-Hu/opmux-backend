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
#[tracing::instrument(level = "debug", skip(request, next))]
pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let start = std::time::Instant::now();
    let config = get_auth_config();

    // Check if development mode is enabled
    if config.is_development_mode() {
        // Development mode: bypass authentication and inject mock context
        let auth_service = AuthService::new();
        let dev_context = auth_service.create_dev_context();
        request.extensions_mut().insert(dev_context);

        let response = next.run(request).await;

        // Log performance metrics for development mode
        let duration = start.elapsed();
        tracing::info!(
            duration_ms = duration.as_millis(),
            mode = "development",
            success = true,
            "Authentication completed (development mode bypass)"
        );

        return Ok(response);
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
            let duration = start.elapsed();
            tracing::warn!(
                duration_ms = duration.as_millis(),
                mode = "production",
                success = false,
                "Authentication failed: Invalid API key"
            );
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Inject AuthContext into request extensions
    request.extensions_mut().insert(auth_context);

    // Continue to next handler
    let response = next.run(request).await;

    // Log performance metrics for successful authentication
    let duration = start.elapsed();
    let is_slow = duration.as_millis() > config.get_slow_threshold_ms() as u128;

    if is_slow {
        tracing::warn!(
            duration_ms = duration.as_millis(),
            mode = "production",
            success = true,
            threshold_ms = config.get_slow_threshold_ms(),
            "Slow authentication detected"
        );
    } else {
        tracing::info!(
            duration_ms = duration.as_millis(),
            mode = "production",
            success = true,
            "Authentication completed"
        );
    }

    Ok(response)
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
