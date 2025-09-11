//! Mock Data Provider for Authentication
//!
//! Provides hardcoded API keys for development and testing
//! Following the same pattern as ingress/mockdata.rs

use super::models::ApiKeyInfo;
use std::collections::HashMap;

/// Mock data provider for authentication system
pub struct MockAuthDataProvider;

impl MockAuthDataProvider {
    /// Get mock API key information by key hash
    /// In real implementation, this would be a database lookup
    pub fn get_api_key_by_hash(key_hash: &str) -> Option<ApiKeyInfo> {
        let mock_keys = Self::get_mock_api_keys();
        mock_keys.get(key_hash).cloned()
    }

    /// Get all mock API keys as a HashMap for quick lookup
    /// Key: SHA-256 hash of the API key
    /// Value: ApiKeyInfo struct
    fn get_mock_api_keys() -> HashMap<String, ApiKeyInfo> {
        let mut keys = HashMap::new();

        // Mock API key: "test-api-key-123"
        // In real implementation, we would store SHA-256 hash
        // For now, we use the plain key as hash for simplicity
        keys.insert(
            "test-api-key-123".to_string(),
            ApiKeyInfo {
                id: "key_001".to_string(),
                client_id: "test-client-456".to_string(),
                key_hash: "test-api-key-123".to_string(), // In real: SHA-256 hash
                name: Some("Test API Key".to_string()),
                created_at: "2025-09-04T00:00:00Z".to_string(),
                last_used_at: Some("2025-09-04T04:00:00Z".to_string()),
                is_active: true,
            },
        );

        // Additional mock API key for testing
        keys.insert(
            "dev-api-key-456".to_string(),
            ApiKeyInfo {
                id: "key_002".to_string(),
                client_id: "dev-client-789".to_string(),
                key_hash: "dev-api-key-456".to_string(),
                name: Some("Development API Key".to_string()),
                created_at: "2025-09-04T01:00:00Z".to_string(),
                last_used_at: None,
                is_active: true,
            },
        );

        keys
    }

    /// Get mock client context for a given client_id
    pub fn get_mock_client_id() -> String {
        "test-client-456".to_string()
    }
}
