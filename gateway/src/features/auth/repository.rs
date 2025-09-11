//! Repository Layer - Data Access for Authentication
//!
//! Handles all data operations for authentication system
//! Currently uses mock data, can be replaced with database implementation later

use super::{mockdata::MockAuthDataProvider, models::ApiKeyInfo};

/// Authentication repository trait
/// Defines interface for data access operations
#[async_trait::async_trait]
pub trait AuthRepository {
    /// Find API key information by key hash
    async fn find_api_key_by_hash(&self, key_hash: &str) -> Option<ApiKeyInfo>;

    /// Update last used timestamp for an API key
    async fn update_last_used(&self, key_id: &str) -> Result<(), String>;
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
    async fn find_api_key_by_hash(&self, key_hash: &str) -> Option<ApiKeyInfo> {
        // Simulate async database lookup
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

        MockAuthDataProvider::get_api_key_by_hash(key_hash)
    }

    /// Mock implementation of updating last used timestamp
    async fn update_last_used(&self, key_id: &str) -> Result<(), String> {
        // Simulate async database update
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

        // In mock implementation, we just log the operation
        println!("Mock: Updated last_used for API key: {}", key_id);
        Ok(())
    }
}

/// Default repository instance
/// In future, this could be configured to use different implementations
pub fn create_auth_repository() -> impl AuthRepository {
    MockAuthRepository::new()
}
