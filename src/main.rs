use anyhow::Context;
use axum::{Json, Router, http::StatusCode, routing::get};
use clap::Parser;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;

#[derive(clap::Parser)]
struct Config {
    #[clap(long, env)]
    db_url: String,
}

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

async fn hello() -> (StatusCode, Json<Response>) {
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

    let app = Router::new().route("/", get(hello));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await
        .context("Failed to start db")?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Failed to bind TCP Listener")?;

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app)
        .await
        .context("axum::serve Failed")?;
    Ok(())
}
