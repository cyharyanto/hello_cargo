use std::net::IpAddr;
use config::{Config, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub log: LogConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

pub fn load_config() -> Result<AppConfig, config::ConfigError> {
    let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
    let run_mode = if run_mode.is_empty() { "development" } else { &run_mode };

    let config = Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name(&format!("config/default.{}", run_mode)).required(false))
        .add_source(File::with_name("config/local").required(false))
        .add_source(Environment::with_prefix("APP").try_parsing(true).separator("_").list_separator(" "))
        .build()?;

    config.try_deserialize()
}
