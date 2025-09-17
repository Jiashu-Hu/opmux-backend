//! Service Layer - Authentication Business Logic
//!
//! Handles all authentication-related business operations
//! This is where the validation logic from middleware/auth.rs is moved to

use super::{
    config::get_auth_config,
    models::{ApiKeyInfo, AuthContext},
    repository::{create_auth_repository, AuthRepository, MockAuthRepository},
};

/// Authentication service
/// Handles all authentication business logic
pub struct AuthService {
    repository: MockAuthRepository,
}

impl Default for AuthService {
    fn default() -> Self {
        Self {
            repository: MockAuthRepository::new(),
        }
    }
}

impl AuthService {
    /// Create new AuthService with default repository
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate API key and return AuthContext if valid
    /// This replaces the hardcoded validation logic from middleware
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn validate_api_key(&self, api_key: &str) -> Option<AuthContext> {
        let start = std::time::Instant::now();
        // Look up API key in repository
        let api_key_info = self.repository.find_api_key_by_hash(api_key).await;

        let duration = start.elapsed();
        let config = get_auth_config();

        match api_key_info {
            Some(info) => {
                // Check if key is active
                if !info.is_active {
                    tracing::debug!(
                        duration_ms = duration.as_millis(),
                        success = false,
                        reason = "inactive_key",
                        "API key validation failed: key is inactive"
                    );
                    return None;
                }

                // Update last used timestamp (fire and forget)
                let key_id = info.id.clone();
                tokio::spawn(async move {
                    let repo = create_auth_repository();
                    if let Err(e) = repo.update_last_used(&key_id).await {
                        tracing::warn!("Failed to update last_used timestamp: {:?}", e);
                    }
                });

                // Log successful validation
                let is_slow =
                    duration.as_millis() > config.get_slow_threshold_ms() as u128;
                if is_slow {
                    tracing::warn!(
                        duration_ms = duration.as_millis(),
                        success = true,
                        threshold_ms = config.get_slow_threshold_ms(),
                        "Slow API key validation detected"
                    );
                } else {
                    tracing::debug!(
                        duration_ms = duration.as_millis(),
                        success = true,
                        "API key validation completed"
                    );
                }

                // Return authentication context
                Some(AuthContext {
                    client_id: info.client_id,
                })
            }
            None => {
                tracing::debug!(
                    duration_ms = duration.as_millis(),
                    success = false,
                    reason = "key_not_found",
                    "API key validation failed: key not found"
                );
                None
            }
        }
    }

    /// Create development mode context
    /// Used when AUTH_DEVELOPMENT_MODE is enabled
    pub fn create_dev_context(&self) -> AuthContext {
        let config = get_auth_config();
        tracing::warn!("🚨 Using development mode authentication bypass");
        tracing::warn!("🚨 Mock client ID: {}", config.get_dev_client_id());

        AuthContext {
            client_id: config.get_dev_client_id().to_string(),
        }
    }

    /// Get API key information (for future API key management endpoints)
    pub async fn get_api_key_info(&self, key_hash: &str) -> Option<ApiKeyInfo> {
        self.repository.find_api_key_by_hash(key_hash).await
    }
}
