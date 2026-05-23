use std::collections::HashMap;

use axum::{Json, Router, routing::get};
use tower_http::trace::TraceLayer;

use crate::{core::AppState, tasks::router::task_routes};

async fn hello() -> Json<HashMap<&'static str, &'static str>> {
    let res = HashMap::from([("message", "pong")]);
    Json(res)
}

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello))
        .nest("/tasks", task_routes())
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
