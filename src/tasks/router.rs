use askama::Template;
use axum::{
    Form, Router,
    extract::State,
    response::{Html, Redirect},
    routing::{get, post},
};
use validator::Validate;

use crate::{
    core::AppState,
    tasks::{
        errors::TaskError,
        models::{CreateTaskQueryParams, CreateTaskRequest},
        queries,
    },
};

type Result<T> = std::result::Result<T, TaskError>;

#[derive(Template)]
#[template(path = "tasks/create.html")]
struct CreateFormTemplate;

async fn show_create_form() -> Result<Html<String>> {
    Ok(Html(CreateFormTemplate.render()?))
}

async fn create(
    State(app_state): State<AppState>,
    Form(fields): Form<CreateTaskRequest>,
) -> Result<Redirect> {
    fields.validate()?;
    queries::create_task(&app_state.db, CreateTaskQueryParams::try_from(fields)?).await?;
    Ok(Redirect::to("/"))
}

pub fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create))
        .route("/create", get(show_create_form))
}
