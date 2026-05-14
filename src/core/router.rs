use axum::{Json, Router, http::StatusCode, routing::get};
use serde::Serialize;

use crate::core::AppState;

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

pub fn app_router(state: AppState) -> Router {
    Router::new().route("/", get(hello)).with_state(state)
}
