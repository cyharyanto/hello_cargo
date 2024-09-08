mod config;
mod logging;

use hello_cargo::{app};
use std::net::SocketAddr;
use tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;

    logging::setup_logging(&config.log.level)?;

    tracing::info!("Starting application");
    tracing::debug!("Loaded configuration: {:?}", config);

    let app = app();

    let addr = SocketAddr::new(config.server.host, config.server.port);
    tracing::info!("Listening on {}", addr);
    tracing::info!("Swagger UI available at http://{}:{}/swagger-ui/", config.server.host, config.server.port);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Application shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    tracing::info!("Received shutdown signal");
}