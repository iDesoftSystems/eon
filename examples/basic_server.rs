use axum::{Router, routing::get};
use eon::http::HttpServer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new("info"))
        .init();

    // Define the router
    let app = Router::new()
        .route("/", get(|| async { "Hello from Eon Axum Runner!" }))
        .route("/health", get(|| async { "OK" }));

    // Initialize and run the server
    let server = HttpServer::from_env(app);

    server.run().await?;

    Ok(())
}
