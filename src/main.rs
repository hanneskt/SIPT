use sipt::{config::AppConfig, start_app};
use std::error::Error;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let config = AppConfig::load()?;

    setup_logging();

    start_app(config).await?;

    Ok(())
}

fn setup_logging() {
    let default_level = if cfg!(debug_assertions) {
        LevelFilter::INFO
    } else {
        LevelFilter::ERROR // or LevelFilter::WARN for release
    };

    let filter = EnvFilter::builder()
        .with_env_var("LOG_LEVEL")
        .with_default_directive(default_level.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
}
