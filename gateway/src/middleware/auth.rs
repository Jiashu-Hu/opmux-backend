// Authentication middleware - protects endpoints with API key validation

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

/// Basic authentication context injected into requests
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub client_id: String,
}

/// Authentication middleware function
/// Validates X-API-Key header and injects AuthContext into request
pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract headers
    let headers = request.headers();
    
    // Get API key from X-API-Key header
    let api_key = match extract_api_key(headers) {
        Some(key) => key,
        None => {
            println!("Authentication failed: Missing X-API-Key header");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    
    // Validate API key (hardcoded for now)
    let auth_context = match validate_api_key(&api_key) {
        Some(context) => context,
        None => {
            println!("Authentication failed: Invalid API key: {}", api_key);
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

/// Validate API key and return AuthContext if valid
/// Currently hardcoded - will be replaced with database lookup in Iteration 2
fn validate_api_key(api_key: &str) -> Option<AuthContext> {
    // Hardcoded test API key
    if api_key == "test-api-key-123" {
        Some(AuthContext {
            client_id: "test-client-456".to_string(),
        })
    } else {
        None
    }
}

/// Axum extractor for AuthContext
/// Allows handlers to easily access authentication context
impl<S> axum::extract::FromRequestParts<S> for AuthContext
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthContext>()
            .cloned()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
