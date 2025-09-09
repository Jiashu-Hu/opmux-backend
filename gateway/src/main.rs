use axum::{
    middleware,
    response::Html,
    routing::{get, post},
    Router,
};
use gateway::{
    features::{health, ingress},
    middleware::auth,
};

#[tokio::main]
async fn main() {
    // Initialize tracing for structured logging
    tracing_subscriber::fmt::init();

    // Create protected routes that require authentication
    let protected_routes = Router::new()
        .route("/api/v1/route", post(ingress::ingress_handler))
        .layer(middleware::from_fn(auth::auth_middleware));

    // Create public routes that don't require authentication
    let public_routes = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health::health_handler));

    // Combine all routes
    let app = Router::new().merge(protected_routes).merge(public_routes);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Gateway server running on http://0.0.0.0:3000");
    tracing::info!("Health check available at http://0.0.0.0:3000/health");
    tracing::info!(
        "Protected ingress endpoint available at http://0.0.0.0:3000/api/v1/route"
    );
    tracing::info!(
        "Authentication required: X-API-Key header with value 'test-api-key-123'"
    );

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Gateway Service</h1><p>Minimal working server</p>")
}
