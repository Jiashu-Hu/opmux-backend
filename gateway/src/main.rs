use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // Create a basic router with a simple endpoint
    let app = Router::new().route("/", get(hello_world));

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Gateway server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Gateway Service</h1><p>Minimal working server</p>")
}
