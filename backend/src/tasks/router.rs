use axum::{Router, extract::State, http::StatusCode, response::Json, routing::get};
use validator::Validate;

use crate::{
    core::AppState,
    tasks::{
        errors::TaskError,
        models::{CreateTask, Task},
        queries::{self},
    },
    users::User,
};

type Result<T> = std::result::Result<T, TaskError>;

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn fetch_all(user: User, State(app_state): State<AppState>) -> Result<Json<Vec<Task>>> {
    let tasks = queries::fetch_all_tasks(&app_state.db, &user.id).await?;
    tracing::debug!(?tasks);
    Ok(Json(tasks))
}

#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn fetch_current(user: User, State(app_state): State<AppState>) -> Result<Json<Vec<Task>>> {
    tracing::debug!("Working");
    let tasks = queries::fetch_current_tasks(&app_state.db, &user.id).await?;
    tracing::debug!(?tasks);
    Ok(Json(tasks))
}

#[tracing::instrument(skip(app_state))]
async fn create(
    user: User,
    State(app_state): State<AppState>,
    Json(fields): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>)> {
    tracing::debug!(?fields);
    fields.validate()?;
    let task = queries::create_task(&app_state.db, fields, &user.id).await?;
    tracing::debug!(?task);
    Ok((StatusCode::CREATED, Json(task)))
}

pub fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(fetch_all).post(create))
        .route("/current", get(fetch_current))
}
