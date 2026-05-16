use askama::Template;
use axum::{Router, http::StatusCode, response::Html, routing::get};

use crate::{
    core::{AppState, errors::AppError},
    tasks::router::task_routes,
};

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate;

async fn hello() -> Result<(StatusCode, Html<String>), AppError> {
    let tmpl = HomeTemplate.render()?;

    Ok((StatusCode::OK, Html(tmpl)))
}

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello))
        .nest("/tasks", task_routes())
        .with_state(state)
}
