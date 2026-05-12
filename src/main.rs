use axum::{Json, Router, http::StatusCode, routing::get};
use serde::Serialize;

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
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
