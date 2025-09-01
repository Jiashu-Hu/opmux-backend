use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use gateway::features::{health, ingress};

#[tokio::main]
async fn main() {
    // Create router with health check and ingress endpoints
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health::health_handler))
        .route("/api/v1/chat", post(ingress::ingress_handler));

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Gateway server running on http://0.0.0.0:3000");
    println!("Health check available at http://0.0.0.0:3000/health");
    println!("Ingress endpoint available at http://0.0.0.0:3000/api/v1/chat");

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Gateway Service</h1><p>Minimal working server</p>")
}
