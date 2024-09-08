use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing::Level;

pub fn setup_logging(log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
    log_level.parse::<Level>()?;

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("hello_cargo={}", log_level)));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}