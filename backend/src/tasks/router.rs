use axum::{Router, extract::State, http::StatusCode, response::Json, routing::get};
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

async fn fetch_all(State(app_state): State<AppState>) -> Result<Json<Vec<Task>>> {
    let tasks = queries::fetch_all_tasks(&app_state.db).await?;
    Ok(Json(tasks))
}

async fn create(
    State(app_state): State<AppState>,
    Json(fields): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<Task>)> {
    fields.validate()?;
    let task =
        queries::create_task(&app_state.db, CreateTaskQueryParams::try_from(fields)?).await?;
    Ok((StatusCode::CREATED, Json(task)))
}

pub fn task_routes() -> Router<AppState> {
    Router::new().route("/", get(fetch_all).post(create))
}
