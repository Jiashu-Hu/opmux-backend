//! Authentication Data Models
//!
//! Contains all data structures used in the authentication system

use serde::{Deserialize, Serialize};

/// Authentication context injected into requests after successful authentication
/// This is moved from middleware/auth.rs to follow 3-layer architecture
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub client_id: String,
}

/// API Key information stored in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    pub id: String,
    pub client_id: String,
    pub key_hash: String, // SHA-256 hash of the actual API key
    pub name: Option<String>,
    pub created_at: String,
    pub last_used_at: Option<String>,
    pub is_active: bool,
}

/// Axum extractor for AuthContext
/// Allows handlers to easily access authentication context
impl<S> axum::extract::FromRequestParts<S> for AuthContext
where
    S: Send + Sync,
{
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthContext>()
            .cloned()
            .ok_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
