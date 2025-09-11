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
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            development_mode: false,
            dev_client_id: "dev-client-123".to_string(),
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

        Self {
            development_mode,
            dev_client_id,
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
    }

    #[test]
    fn test_development_mode_detection() {
        let config = AuthConfig {
            development_mode: true,
            dev_client_id: "test-client".to_string(),
        };
        assert!(config.is_development_mode());
        assert_eq!(config.get_dev_client_id(), "test-client");
    }
}
