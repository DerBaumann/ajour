use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use task_manager::core::{
    AppState, config::Config, get_db_pool, log::setup_logging, router::app_router, serve_app,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenvy::dotenv().ok();

    setup_logging();

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
