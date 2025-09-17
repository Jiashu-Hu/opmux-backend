//! Authentication Configuration
//!
//! Handles environment variable configuration for authentication system
//! Following organic development principles - add configuration as needed

use std::env;

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// Enable development mode bypass
    /// When true, allows requests without API keys and injects mock context
    pub development_mode: bool,

    /// Mock client ID to use in development mode
    pub dev_client_id: String,

    /// Slow operation threshold in milliseconds
    /// Operations taking longer than this will be logged as warnings
    pub slow_operation_threshold_ms: u64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            development_mode: false,
            dev_client_id: "dev-client-123".to_string(),
            slow_operation_threshold_ms: 10, // 10ms threshold for mock data
        }
    }
}

impl AuthConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let development_mode = env::var("AUTH_DEVELOPMENT_MODE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        let dev_client_id = env::var("AUTH_DEV_CLIENT_ID")
            .unwrap_or_else(|_| "dev-client-123".to_string());

        let slow_operation_threshold_ms = env::var("AUTH_SLOW_THRESHOLD_MS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u64>()
            .unwrap_or(10);

        Self {
            development_mode,
            dev_client_id,
            slow_operation_threshold_ms,
        }
    }

    /// Check if development mode is enabled
    pub fn is_development_mode(&self) -> bool {
        self.development_mode
    }

    /// Get development client ID
    pub fn get_dev_client_id(&self) -> &str {
        &self.dev_client_id
    }

    /// Get slow operation threshold in milliseconds
    pub fn get_slow_threshold_ms(&self) -> u64 {
        self.slow_operation_threshold_ms
    }

    /// Log configuration warnings
    pub fn log_warnings(&self) {
        if self.development_mode {
            tracing::warn!("🚨 AUTH_DEVELOPMENT_MODE is ENABLED");
            tracing::warn!("🚨 Authentication is BYPASSED for development");
            tracing::warn!("🚨 This should NEVER be enabled in production");
            tracing::warn!("🚨 Mock client ID: {}", self.dev_client_id);
        } else {
            tracing::info!("✅ Authentication is ENABLED (production mode)");
        }
    }
}

use std::sync::OnceLock;

/// Global configuration instance
static CONFIG: OnceLock<AuthConfig> = OnceLock::new();

/// Get global authentication configuration
/// Initializes from environment variables on first call
pub fn get_auth_config() -> &'static AuthConfig {
    CONFIG.get_or_init(|| {
        let config = AuthConfig::from_env();
        config.log_warnings();
        config
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AuthConfig::default();
        assert!(!config.development_mode);
        assert_eq!(config.dev_client_id, "dev-client-123");
        assert_eq!(config.slow_operation_threshold_ms, 10);
    }

    #[test]
    fn test_development_mode_detection() {
        let config = AuthConfig {
            development_mode: true,
            dev_client_id: "test-client".to_string(),
            slow_operation_threshold_ms: 20,
        };
        assert!(config.is_development_mode());
        assert_eq!(config.get_dev_client_id(), "test-client");
        assert_eq!(config.get_slow_threshold_ms(), 20);
    }
}
