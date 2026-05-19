use askama::Template;
use axum::{
    Form, Router,
    extract::State,
    response::{Html, Redirect},
    routing::get,
};
use validator::Validate;

use crate::{
    core::AppState,
    tasks::{
        errors::TaskError,
        models::{CreateTaskQueryParams, CreateTaskRequest, Task},
        queries,
    },
};

type Result<T> = std::result::Result<T, TaskError>;

#[derive(Template)]
#[template(path = "tasks/index.html")]
struct TaskListTemplate {
    tasks: Vec<Task>,
}

async fn show_task_list(State(app_state): State<AppState>) -> Result<Html<String>> {
    let tasks = queries::fetch_all_tasks(&app_state.db).await?;
    let tmpl = TaskListTemplate { tasks };

    Ok(Html(tmpl.render()?))
}

async fn create(
    State(app_state): State<AppState>,
    Form(fields): Form<CreateTaskRequest>,
) -> Result<Redirect> {
    dbg!(&fields);
    fields.validate()?;
    queries::create_task(&app_state.db, CreateTaskQueryParams::try_from(fields)?).await?;
    Ok(Redirect::to("/tasks"))
}

pub fn task_routes() -> Router<AppState> {
    Router::new().route("/", get(show_task_list).post(create))
}
