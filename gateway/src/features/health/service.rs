// Service Layer - Business logic abstraction

use super::{error::HealthError, repository::HealthRepository};
use serde::{Deserialize, Serialize};

/// Response structure for health check endpoints.
///
/// This structure represents the standardized response format for health checks,
/// providing both status information and timing data for monitoring systems.
#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    /// Current health status of the system.
    ///
    /// Possible values:
    /// - `"healthy"` - All systems operational
    /// - `"unhealthy"` - One or more critical systems failing
    /// - `"degraded"` - Some non-critical systems failing (future use)
    pub status: String,

    /// ISO 8601 timestamp of when the health check was performed.
    ///
    /// Format: `YYYY-MM-DDTHH:MM:SS.sssZ`
    /// Example: `"2025-09-01T16:53:30.625665+00:00"`
    pub timestamp: String,
}

/// Service for health check business logic and orchestration.
///
/// This service coordinates health check operations across multiple system components,
/// aggregates health status information, and applies business rules to determine
/// overall system health. It acts as the business logic layer between the HTTP
/// handlers and the data access layer.
///
/// # Architecture
///
/// The service follows the 3-layer architecture pattern:
/// - Receives requests from the handler layer
/// - Orchestrates calls to the repository layer
/// - Applies business logic to determine health status
/// - Returns structured responses to the handler layer
pub struct HealthService {
    /// Repository for accessing health check data and system status.
    repository: HealthRepository,
}

impl HealthService {
    /// Creates a new instance of the health service.
    ///
    /// Initializes the service with a new repository instance for data access.
    ///
    /// # Returns
    ///
    /// A new `HealthService` instance ready to perform health checks.
    pub fn new() -> Self {
        Self { repository: HealthRepository::new() }
    }

    /// Performs a comprehensive health check of the system.
    ///
    /// This method orchestrates the entire health check process by:
    /// 1. Retrieving system status from the repository layer
    /// 2. Applying business logic to determine overall health
    /// 3. Generating a timestamped response
    ///
    /// # Returns
    ///
    /// - `Ok(HealthResponse)` - Health check completed successfully
    /// - `Err(HealthError)` - Health check failed due to system issues
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - System status check fails (`HealthError::SystemStatusCheckFailed`)
    /// - Health aggregation fails (`HealthError::HealthAggregationFailed`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let service = HealthService::new();
    /// match service.check_health().await {
    ///     Ok(response) => {
    ///         println!("Health: {} at {}", response.status, response.timestamp);
    ///     },
    ///     Err(e) => eprintln!("Health check failed: {}", e),
    /// }
    /// ```
    pub async fn check_health(&self) -> Result<HealthResponse, HealthError> {
        // Get system status from repository
        let system_status = self.repository.get_system_status().await?;

        // Business logic: determine overall health
        let status =
            if system_status.is_healthy { "healthy".to_string() } else { "unhealthy".to_string() };

        Ok(HealthResponse { status, timestamp: chrono::Utc::now().to_rfc3339() })
    }
}
