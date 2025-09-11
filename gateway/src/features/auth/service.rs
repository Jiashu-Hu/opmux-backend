//! Service Layer - Authentication Business Logic
//!
//! Handles all authentication-related business operations
//! This is where the validation logic from middleware/auth.rs is moved to

use super::{
    models::{ApiKeyInfo, AuthContext},
    repository::{create_auth_repository, AuthRepository, MockAuthRepository},
};

/// Authentication service
/// Handles all authentication business logic
pub struct AuthService {
    repository: MockAuthRepository,
}

impl AuthService {
    /// Create new AuthService with default repository
    pub fn new() -> Self {
        Self {
            repository: MockAuthRepository::new(),
        }
    }

    /// Validate API key and return AuthContext if valid
    /// This replaces the hardcoded validation logic from middleware
    pub async fn validate_api_key(&self, api_key: &str) -> Option<AuthContext> {
        // Look up API key in repository
        let api_key_info = self.repository.find_api_key_by_hash(api_key).await?;

        // Check if key is active
        if !api_key_info.is_active {
            tracing::warn!("Authentication failed: API key is inactive: {}", api_key);
            return None;
        }

        // Update last used timestamp (fire and forget)
        let key_id = api_key_info.id.clone();
        tokio::spawn(async move {
            let repo = create_auth_repository();
            if let Err(e) = repo.update_last_used(&key_id).await {
                tracing::warn!("Failed to update last_used timestamp: {:?}", e);
            }
        });

        // Return authentication context
        Some(AuthContext {
            client_id: api_key_info.client_id,
        })
    }

    /// Get API key information (for future API key management endpoints)
    pub async fn get_api_key_info(&self, key_hash: &str) -> Option<ApiKeyInfo> {
        self.repository.find_api_key_by_hash(key_hash).await
    }
}
