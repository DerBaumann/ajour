use anyhow::Context;
use askama::Template;
use axum::{Router, http::StatusCode, response::Html, routing::get};

use crate::core::{AppState, errors::AnyhowError};

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate;

async fn hello() -> Result<(StatusCode, Html<String>), AnyhowError> {
    let tmpl = HomeTemplate.render().context("Failed to render template")?;

    Ok((StatusCode::OK, Html(tmpl)))
}

pub fn app_router(state: AppState) -> Router {
    Router::new().route("/", get(hello)).with_state(state)
}
