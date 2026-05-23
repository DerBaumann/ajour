use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use task_manager::core::{AppState, config::Config, get_db_pool, router::app_router, serve_app};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenvy::dotenv().ok();

    // env_logger::init();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!("panic: {:?}", panic_info)
    }));

    let config = Config::parse();
    let port = config.port;

    let pool = get_db_pool(&config.database_url)
        .await
        .context("Failed to start db")?;

    let state = AppState {
        config: Arc::new(config),
        db: pool,
    };
    let app = app_router(state);

    serve_app(app, port).await?;

    Ok(())
}
