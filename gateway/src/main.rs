use axum::{
    middleware,
    response::Html,
    routing::{get, post},
    Router,
};
use gateway::{
    core::config::get_config,
    features::{
        executor::config::ExecutorConfig,
        executor::service::ExecutorService,
        health,
        ingress::{self, AppState},
    },
    middleware::auth,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Initialize tracing for structured logging
    tracing_subscriber::fmt::init();

    // Initialize configuration (logs all settings including warnings)
    let config = get_config();

    // Initialize ExecutorService for LLM execution
    tracing::info!("Initializing ExecutorService...");
    let executor_config = ExecutorConfig::from_env();
    let executor_service = match ExecutorService::from_config(executor_config.clone()) {
        Ok(service) => Arc::new(service),
        Err(e) => {
            tracing::error!("Fatal: Failed to initialize ExecutorService: {}", e);
            std::process::exit(1);
        }
    };
    tracing::info!(
        "ExecutorService initialized with {} vendors",
        executor_service.vendor_count()
    );

    // Create application state with all shared services
    let app_state = AppState { executor_service };

    // Create protected routes that require authentication
    let protected_routes = Router::new()
        .route("/api/v1/route", post(ingress::ingress_handler))
        .layer(middleware::from_fn(auth::auth_middleware))
        .with_state(app_state);

    // Create public routes that don't require authentication
    let public_routes = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health::health_handler));

    // Combine all routes
    let app = Router::new().merge(protected_routes).merge(public_routes);

    // Start the server
    let listener = tokio::net::TcpListener::bind(config.server.bind_address)
        .await
        .unwrap();
    tracing::info!(
        "Gateway server running on http://{}",
        config.server.bind_address
    );
    tracing::info!(
        "Health check available at http://{}/health",
        config.server.bind_address
    );
    tracing::info!(
        "Protected ingress endpoint available at http://{}/api/v1/route",
        config.server.bind_address
    );

    if config.auth.development_mode {
        tracing::info!("🚨 Development mode: Authentication is BYPASSED");
        tracing::info!("🚨 No API key required for testing");
    } else {
        tracing::info!(
            "Authentication required: X-API-Key header with value 'test-api-key-123'"
        );
    }

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Gateway Service</h1><p>Minimal working server</p>")
}
