// Service Layer - Business logic abstraction

use super::{error::HealthError, repository::HealthRepository};
use crate::features::executor::service::ExecutorService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

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

    /// Application version from Cargo.toml
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Uptime in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime_seconds: Option<u64>,
}

/// Response structure for readiness check endpoints.
///
/// Used by Kubernetes readiness probes and load balancers to determine
/// if the service is ready to accept traffic.
#[derive(Serialize, Deserialize)]
pub struct ReadinessResponse {
    /// Overall readiness status ("ready" or "not_ready")
    pub status: String,

    /// ISO 8601 timestamp
    pub timestamp: String,

    /// Status of dependencies (executor, database, etc.)
    pub dependencies: DependencyStatus,
}

/// Status of a dependency (e.g., executor service, database)
#[derive(Serialize, Deserialize, Clone)]
pub struct DependencyStatus {
    /// Status: "healthy" or "unhealthy"
    pub status: String,

    /// Number of available vendors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_count: Option<usize>,

    /// Latency in milliseconds (for connectivity checks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,

    /// Error message if unhealthy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Cached health check result
#[derive(Clone)]
struct CachedHealthResult {
    /// Health check result (Ok if healthy, Err with message if unhealthy)
    result: Result<(), String>,
    /// Timestamp when the result was cached
    cached_at: Instant,
}

/// Health check configuration
pub struct HealthConfig {
    /// Timeout for health checks in seconds
    pub timeout: u64,
    /// Cache TTL for health check results in seconds
    pub cache_ttl_secs: u64,
}

impl HealthConfig {
    /// Creates configuration from environment variables
    pub fn from_env() -> Self {
        let timeout = std::env::var("HEALTH_CHECK_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2);

        let cache_ttl_secs = std::env::var("HEALTH_CHECK_CACHE_TTL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5); // Default: 5 seconds

        Self {
            timeout,
            cache_ttl_secs,
        }
    }
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

    /// Executor service for checking LLM vendor health
    executor_service: Option<Arc<ExecutorService>>,

    /// Health check configuration
    config: HealthConfig,

    /// Application start time for uptime calculation
    start_time: std::time::Instant,

    /// Cache for health check results
    health_cache: Arc<RwLock<Option<CachedHealthResult>>>,
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
        Self {
            repository: HealthRepository::new(),
            executor_service: None,
            config: HealthConfig::from_env(),
            start_time: std::time::Instant::now(),
            health_cache: Arc::new(RwLock::new(None)),
        }
    }

    /// Creates a new instance with executor service dependency.
    ///
    /// Used for readiness checks that need to verify LLM vendor connectivity.
    pub fn with_executor(executor_service: Arc<ExecutorService>) -> Self {
        Self {
            repository: HealthRepository::new(),
            executor_service: Some(executor_service),
            config: HealthConfig::from_env(),
            start_time: std::time::Instant::now(),
            health_cache: Arc::new(RwLock::new(None)),
        }
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
    /// ```rust,no_run
    /// use gateway::features::health::service::HealthService;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let service = HealthService::new();
    ///     match service.check_health().await {
    ///         Ok(response) => {
    ///             println!("Health: {} at {}", response.status, response.timestamp);
    ///         },
    ///         Err(e) => eprintln!("Health check failed: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn check_health(&self) -> Result<HealthResponse, HealthError> {
        // Get system status from repository
        let system_status = self.repository.get_system_status().await?;

        // Business logic: determine overall health
        let status = if system_status.is_healthy {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        };

        Ok(HealthResponse {
            status,
            timestamp: chrono::Utc::now().to_rfc3339(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
            uptime_seconds: Some(self.start_time.elapsed().as_secs()),
        })
    }

    /// Checks if a cached result is still valid.
    fn is_cache_valid(&self, cached: &CachedHealthResult) -> bool {
        cached.cached_at.elapsed().as_secs() < self.config.cache_ttl_secs
    }

    /// Performs health check with caching.
    ///
    /// Checks cache first, then performs actual health check if cache is expired.
    ///
    /// # Caching Strategy
    /// - Only successful health checks are cached
    /// - Failed health checks are NOT cached (allows fast recovery)
    /// - This prevents transient failures from keeping the service "not ready"
    ///   for the entire cache TTL duration
    async fn check_executor_health(&self) -> Result<(), String> {
        // Check cache first
        {
            let cache = self.health_cache.read().await;
            if let Some(cached) = cache.as_ref() {
                if self.is_cache_valid(cached) {
                    return cached.result.clone();
                }
            }
        }

        // Cache miss or expired, perform actual health check
        let executor = self
            .executor_service
            .as_ref()
            .ok_or_else(|| "No executor service configured".to_string())?;

        let result = executor
            .check_all_vendors_health(self.config.timeout)
            .await
            .map_err(|e| {
                // Log detailed error server-side for debugging
                tracing::warn!(
                    error = %e,
                    "Health check failed for executor service"
                );
                // Return generic error message to clients (security: don't leak internal details)
                "Service dependencies unavailable".to_string()
            });

        // Only cache successful results (failures are not cached)
        if result.is_ok() {
            let mut cache = self.health_cache.write().await;
            *cache = Some(CachedHealthResult {
                result: result.clone(),
                cached_at: Instant::now(),
            });
        }

        result
    }

    /// Checks readiness of the service and its dependencies.
    ///
    /// Used by Kubernetes readiness probes and load balancers.
    /// Returns 200 OK if ready, 503 if not ready.
    ///
    /// # Implementation
    ///
    /// This method performs actual health checks on all configured LLM vendors:
    /// - Calls vendor APIs to verify connectivity and credentials
    /// - Uses caching (default 5 seconds TTL) to avoid excessive API calls
    /// - Returns ready if at least one vendor is healthy
    /// - Returns not_ready if all vendors are unhealthy or unreachable
    ///
    /// # Returns
    ///
    /// - `Ok(ReadinessResponse)` - Readiness check completed
    /// - `Err(HealthError)` - Check failed
    pub async fn check_readiness(&self) -> Result<ReadinessResponse, HealthError> {
        let timestamp = chrono::Utc::now().to_rfc3339();

        // If no executor service is configured, consider it ready
        // (service can still handle health checks and other non-LLM endpoints)
        let Some(executor) = &self.executor_service else {
            return Ok(ReadinessResponse {
                status: "ready".to_string(),
                timestamp,
                dependencies: DependencyStatus {
                    status: "healthy".to_string(),
                    vendor_count: None,
                    latency_ms: None,
                    error: None,
                },
            });
        };

        // Perform actual health check with timing
        let start = Instant::now();
        let health_result = self.check_executor_health().await;
        let latency_ms = start.elapsed().as_millis() as u64;

        let vendor_count = executor.vendor_count();

        match health_result {
            Ok(()) => {
                // At least one vendor is healthy
                Ok(ReadinessResponse {
                    status: "ready".to_string(),
                    timestamp,
                    dependencies: DependencyStatus {
                        status: "healthy".to_string(),
                        vendor_count: Some(vendor_count),
                        latency_ms: Some(latency_ms),
                        error: None,
                    },
                })
            }
            Err(error_msg) => {
                // All vendors are unhealthy
                Ok(ReadinessResponse {
                    status: "not_ready".to_string(),
                    timestamp,
                    dependencies: DependencyStatus {
                        status: "unhealthy".to_string(),
                        vendor_count: Some(vendor_count),
                        latency_ms: Some(latency_ms),
                        error: Some(error_msg),
                    },
                })
            }
        }
    }
}

impl Default for HealthService {
    fn default() -> Self {
        Self::new()
    }
}
