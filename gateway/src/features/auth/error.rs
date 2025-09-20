//! Authentication Feature Error Types
//!
//! Errors specific to authentication operations following the business operation model

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;

/// Errors specific to authentication operations.
/// Each variant corresponds to a failed business operation.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("API key validation failed")]
    ApiKeyValidationFailed,

    #[error("API key is inactive")]
    ApiKeyInactive,

    #[error("Repository operation failed: {0}")]
    RepositoryOperationFailed(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            // Client errors are mapped to 4xx status codes.
            Self::ApiKeyValidationFailed | Self::ApiKeyInactive => (
                StatusCode::UNAUTHORIZED,
                "Authentication failed".to_string(),
            ),

            // Server-side business logic failures are mapped to 5xx status codes.
            // Return a generic message to the user for security.
            Self::RepositoryOperationFailed(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal error occurred".to_string(),
            ),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body, response::IntoResponse};

    #[tokio::test]
    async fn api_key_validation_failed_maps_to_401() {
        let err = AuthError::ApiKeyValidationFailed;
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let bytes = body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let s = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(s.contains("Authentication failed"));
    }

    #[tokio::test]
    async fn api_key_inactive_maps_to_401() {
        let err = AuthError::ApiKeyInactive;
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let bytes = body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let s = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(s.contains("Authentication failed"));
    }

    #[tokio::test]
    async fn repository_operation_failed_maps_to_500_generic_message() {
        let err = AuthError::RepositoryOperationFailed("db timeout".into());
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let bytes = body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let s = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(s.contains("internal error"));
    }
}
