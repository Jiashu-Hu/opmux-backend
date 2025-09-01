// Service Layer - Business logic abstraction

use super::{error::HealthError, repository::HealthRepository};
use serde::{Deserialize, Serialize};

// Data models
#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

// Service struct - handles business logic and orchestration
pub struct HealthService {
    repository: HealthRepository,
}

impl HealthService {
    pub fn new() -> Self {
        Self { repository: HealthRepository::new() }
    }

    // Business logic method - orchestrates health check
    pub async fn check_health(&self) -> Result<HealthResponse, HealthError> {
        // Get system status from repository
        let system_status = self.repository.get_system_status().await?;

        // Business logic: determine overall health
        let status =
            if system_status.is_healthy { "healthy".to_string() } else { "unhealthy".to_string() };

        Ok(HealthResponse { status, timestamp: chrono::Utc::now().to_rfc3339() })
    }
}
