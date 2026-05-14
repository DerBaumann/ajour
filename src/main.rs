use std::sync::Arc;

use anyhow::Context;
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use clap::Parser;
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

// TODO: Split main up

// TODO: Add Port to config
#[derive(Debug, clap::Parser)]
struct Config {
    #[arg(long, env)]
    db_url: String,
}

#[derive(Debug, Clone)]
struct AppContext {
    config: Arc<Config>,
    db: PgPool,
}

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

async fn hello(State(ctx): State<AppContext>) -> (StatusCode, Json<Response>) {
    // TODO: Delete me!
    println!("{}", ctx.db.size());
    let response = Response {
        message: "Hello, World!",
    };

    (StatusCode::OK, Json(response))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenvy::dotenv().ok();

    let config = Config::parse();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await
        .context("Failed to start db")?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let ctx = AppContext {
        config: Arc::new(config),
        db: pool,
    };

    let app = Router::new().route("/", get(hello)).with_state(ctx);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Failed to bind TCP Listener")?;

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app)
        .await
        .context("axum::serve Failed")?;
    Ok(())
}
