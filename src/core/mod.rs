use crate::core::config::Config;
use anyhow::Context;
use axum::Router;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};

pub mod config;
pub mod router;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: PgPool,
}

pub async fn get_db_pool(url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .context("Failed to start db")?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub async fn serve_app(router: Router, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("Failed to bind TCP Listener")?;

    println!("Listening on {}", addr);
    axum::serve(listener, router)
        .await
        .context("axum::serve Failed")?;

    Ok(())
}
