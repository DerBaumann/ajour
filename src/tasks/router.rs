use anyhow::Context;
use askama::Template;
use axum::{
    Form, Router,
    extract::State,
    response::{Html, Redirect},
    routing::{get, post},
};
use validator::Validate;

use crate::{
    core::{
        AppState,
        errors::{AnyhowError, AppError},
    },
    tasks::{models::CreateTaskRequest, queries},
};

#[derive(Template)]
#[template(path = "tasks/create.html")]
struct CreateFormTemplate;

async fn show_create_form() -> Result<Html<String>, AppError> {
    Ok(Html(CreateFormTemplate.render()?))
}

async fn create(
    State(app_state): State<AppState>,
    Form(fields): Form<CreateTaskRequest>,
) -> Result<Redirect, AnyhowError> {
    fields.validate().context("Invalid task data!")?;
    queries::create_task(&app_state.db, fields.into_query_params()?).await?;
    Ok(Redirect::to("/"))
}

pub fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create))
        .route("/create", get(show_create_form))
}
