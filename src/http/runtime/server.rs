use axum::Router;
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;

/// A runner for Axum services.
///
/// This struct provides a simple way to start an Axum server with
/// built-in graceful shutdown support and structured logging.
pub struct HttpServer {
    router: Router,
    addr: SocketAddr,
}

impl HttpServer {
    /// Creates a new `HttpServer` instance.
    ///
    /// # Arguments
    ///
    /// * `router` - The Axum router containing the service routes.
    /// * `addr` - The socket address the server should bind to.
    pub fn new(router: Router, addr: SocketAddr) -> Self {
        Self { router, addr }
    }

    /// Creates a new `HttpServer` instance from environment variables.
    ///
    /// This constructor looks for the `PORT` or `APP_PORT` environment variables.
    /// If neither is found, it defaults to port `3000`. It binds to `0.0.0.0`
    /// to be accessible from outside the container/host.
    pub fn from_env(router: Router) -> Self {
        let port = std::env::var("PORT")
            .or_else(|_| std::env::var("APP_PORT"))
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3000);

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        Self::new(router, addr)
    }

    /// Runs the server until a termination signal is received.
    ///
    /// This method starts the server and waits for either a SIGINT (Ctrl+C)
    /// or SIGTERM signal before starting the graceful shutdown process.
    ///
    /// # Errors
    ///
    /// Returns an error if the server fails to start or bind to the address.
    pub async fn run(self) -> Result<(), std::io::Error> {
        let listener = tokio::net::TcpListener::bind(&self.addr).await?;

        info!("Server listening on http://{}", self.addr);

        axum::serve(listener, self.router)
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        info!("Server stopped successfully");
        Ok(())
    }
}

/// Helper function to handle termination signals for graceful shutdown.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, starting graceful shutdown");
        },
        _ = terminate => {
            info!("Received SIGTERM, starting graceful shutdown");
        },
    }
}
