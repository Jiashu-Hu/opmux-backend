use axum::response::{IntoResponse, Response};
// Import feature error modules
use crate::features::{health, ingress};

/// The single, top-level error type for the entire application.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // Add feature errors here using #[from].
    // The `transparent` attribute passes the Display implementation up.
    #[error(transparent)]
    Health(#[from] health::error::HealthError),

    #[error(transparent)]
    Ingress(#[from] ingress::error::IngressError),
    // --- Add other features below as they are created ---
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // In a real app, use a structured logger like tracing.
        tracing::error!("An error occurred: {:?}", self);

        // Delegate response generation to the underlying feature error.
        match self {
            AppError::Health(e) => e.into_response(),
            AppError::Ingress(e) => e.into_response(),
        }
    }
}
