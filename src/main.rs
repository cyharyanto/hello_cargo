use hello_cargo::app;
use std::net::SocketAddr;
use tracing;
use axum::middleware::map_response;
use axum::response::Response;
use tower_http::trace::TraceLayer;
use tracing::{info, debug};
use std::sync::Arc;

mod config;
mod logging;

use hello_cargo::repositories::postgres_repository::PostgresUserRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;

    logging::setup_logging(&config.log.level)?;

    info!("Starting application");
    debug!("Loaded configuration: {:?}", config);

    let user_repository = Arc::new(PostgresUserRepository::new(&config.database.url)) as Arc<dyn hello_cargo::repositories::UserRepository>;

    let app = app(user_repository).layer(TraceLayer::new_for_http())
        .layer(map_response(logging_middleware));

    let addr = SocketAddr::new(config.server.host, config.server.port);
    info!("Listening on {}", addr);
    info!("Swagger UI available at http://{}:{}/swagger-ui/", config.server.host, config.server.port);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Application shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    info!("Received shutdown signal");
}

async fn logging_middleware(response: Response) -> Response {
    debug!("Response status: {}", response.status());
    response
}