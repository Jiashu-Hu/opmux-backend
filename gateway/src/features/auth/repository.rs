//! Repository Layer - Data Access for Authentication
//!
//! Handles all data operations for authentication system
//! Currently uses mock data, can be replaced with database implementation later

use super::{error::AuthError, mockdata::MockAuthDataProvider, models::ApiKeyInfo};

/// Authentication repository trait
/// Defines interface for data access operations
#[async_trait::async_trait]
pub trait AuthRepository {
    /// Find API key information by key hash
    async fn find_api_key_by_hash(&self, key_hash: &str) -> Option<ApiKeyInfo>;

    /// Update last used timestamp for an API key
    async fn update_last_used(&self, key_id: &str) -> Result<(), AuthError>;
}

/// Mock implementation of AuthRepository
/// Uses hardcoded data for development and testing
pub struct MockAuthRepository;

impl MockAuthRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl AuthRepository for MockAuthRepository {
    /// Find API key by hash using mock data
    #[tracing::instrument(level = "debug", skip(self))]
    async fn find_api_key_by_hash(&self, key_hash: &str) -> Option<ApiKeyInfo> {
        let start = std::time::Instant::now();

        // Simulate async database lookup
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

        let result = MockAuthDataProvider::get_api_key_by_hash(key_hash);

        // Log repository performance
        let duration = start.elapsed();
        tracing::debug!(
            duration_ms = duration.as_millis(),
            found = result.is_some(),
            "Repository query completed"
        );

        result
    }

    /// Mock implementation of updating last used timestamp
    #[tracing::instrument(level = "debug", skip(self))]
    async fn update_last_used(&self, key_id: &str) -> Result<(), AuthError> {
        let start = std::time::Instant::now();

        // Simulate async database update
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

        // In mock implementation, we just log the operation
        let duration = start.elapsed();
        tracing::debug!(
            duration_ms = duration.as_millis(),
            key_id = key_id,
            "Repository update completed"
        );

        Ok(())
    }
}

/// Default repository instance
/// In future, this could be configured to use different implementations
pub fn create_auth_repository() -> impl AuthRepository {
    MockAuthRepository::new()
}
